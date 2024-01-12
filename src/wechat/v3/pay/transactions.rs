mod close_transaction;
mod common;
mod create_merged_transaction;
mod create_transaction;
mod order_detail;
mod transaction;

pub mod notify;
pub use create_merged_transaction::{
    ContactInfo, CreateMergedTransactionsParams, SubOrder, WechatMergedSubOrderAmount,
};

pub use close_transaction::{CloseCombinedOrderParams, CloseOrderParams, CloseSubOrderParams};
pub use common::{SalesSceneInfo, SettlementInfo, StoreInfo};
pub use create_transaction::{
    AppPrePayResponse, CreateTransactionParams, TransactionSettlementInfo, WechatPrePayAmount,
};
pub use order_detail::*;
pub use transaction::*;

use super::types::WechatPayMethod;
use crate::{
    errors::Kind,
    wechat::{
        conf::WechatConfig,
        crypto::decode_certificate,
        http,
        types::{notify::WechatPayNotify, signature_header::SignatureHeader},
        verify_notify_sign,
    },
    Error, Result,
};
use reqwest::Method;
use serde::de::DeserializeOwned;

const TRANSACTIONS: &str = if cfg!(feature = "platform") {
    "/v3/pay/partner/transactions"
} else {
    "/v3/pay/transactions"
};

const COMBINE_TRANSACTIONS: &str = "/v3/combine-transactions";

/// 单笔交易
pub async fn create_transaction<T: DeserializeOwned>(
    conf: &WechatConfig,
    method: &WechatPayMethod,
    params: CreateTransactionParams,
) -> crate::Result<T> {
    http::upsert(
        conf,
        Method::POST,
        &format!("{TRANSACTIONS}/{}", method.channel_name()),
        &params,
    )
    .await
}

pub async fn create_combined_transactions<T: DeserializeOwned>(
    conf: &WechatConfig,
    method: &WechatPayMethod,
    params: &CreateMergedTransactionsParams,
) -> crate::Result<T> {
    http::upsert(
        conf,
        Method::POST,
        &format!("{COMBINE_TRANSACTIONS}/{}", method.channel_name()),
        &params,
    )
    .await
}

pub enum FindTransactionBy {
    CustomId(String),
    TransactionId(String),
    // CombinedCustomId(String)
}

pub async fn find_transaction_by(
    conf: &WechatConfig,
    by: FindTransactionBy,
    mch_id: String,
    #[cfg(feature = "platform")] sub_mch_id: String,
) -> Result<Transaction> {
    let path = match by {
        FindTransactionBy::CustomId(id) => format!("{TRANSACTIONS}/out-trade-no/{id}"),
        FindTransactionBy::TransactionId(id) => format!("{TRANSACTIONS}/transactions/id/{id}"),
    };

    cfg_if::cfg_if! {
        if #[cfg(feature="platform")] {
            let query = format!("?sp_mchid={mch_id}&sub_mchid={sub_mch_id}");
        } else {
            let query = format!("?mchid={mch_id}");
        }
    }

    http::get(conf, &path, Some(&query)).await
}

pub async fn find_combine_transaction_by_id<T: Into<String>>(
    conf: &WechatConfig,
    id: T,
) -> Result<CombineTransaction> {
    http::get(
        conf,
        &format!("{COMBINE_TRANSACTIONS}/out-trade-no/{}", id.into()),
        None,
    )
    .await
}

pub async fn close_order(conf: &WechatConfig, id: &str, payload: &CloseOrderParams) -> Result<()> {
    http::upsert(
        conf,
        Method::POST,
        &format!("{TRANSACTIONS}/out-trade-no/{id}/close"),
        payload,
    )
    .await
}

pub async fn close_combined_order(
    conf: &WechatConfig,
    id: &str,
    payload: &CloseCombinedOrderParams,
) -> crate::Result<()> {
    http::upsert(
        conf,
        Method::POST,
        &format!("{COMBINE_TRANSACTIONS}/out-trade-no/{id}/close"),
        payload,
    )
    .await
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct DecodedNotifyResponse<T> {
//     pub raw_data: WechatPayNotifyRequest,
//     pub resource: T,
// }

pub async fn decode_notify<T>(
    api_key: &[u8],
    header: &SignatureHeader,
    notify: &WechatPayNotify,
) -> Result<T>
where
    T: DeserializeOwned,
{
    let data_str = serde_json::to_string(notify)?;
    if !verify_notify_sign(header, &data_str) {
        return Err(Error::from_kind(Kind::InvalidSignature));
    }

    // let notify = serde_json::from_str::<WechatPayNotifyRequest>(notify)?;
    let resource = notify.resource.to_owned();

    let buffer = decode_certificate(
        notify
            .resource
            .associated_data
            .clone()
            .unwrap_or_default()
            .as_bytes(),
        header.nonce.as_bytes(),
        api_key,
        resource.ciphertext.as_bytes(),
    )?;

    let resource = serde_json::from_slice::<T>(&buffer)?;

    Ok(resource)

    // Ok(DecodedNotifyResponse {
    //     raw_data: notify,
    //     resource,
    // })
}
