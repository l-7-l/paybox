pub mod account;
mod applyment;
mod contact;
mod finance_type;
mod id_card;
mod id_doc;
mod sales_scene;
mod settlement_account;
mod sino_id;
mod ubo;

pub use applyment::*;
pub use finance_type::*;
pub use id_card::IdCardInfo;
pub use id_doc::IdDocInfo;
pub use sales_scene::SalesSceneInfo;
pub use settlement_account::*;
pub use sino_id::SinoIdKind;
pub use ubo::UBO;

use crate::Result;

use super::{conf::WechatConfig, http};
use reqwest::Method;

pub async fn new_merchant(
    conf: &WechatConfig,
    params: &NewSubMerchantApplyment,
) -> crate::Result<NewApplymentResponse> {
    http::upsert(conf, Method::POST, "/v3/ecommerce/applyments", params).await
}

pub enum FindApplymentBy {
    CustomId(String),
    Id(i64),
}

pub async fn find_applyment(
    conf: &WechatConfig,
    by: FindApplymentBy,
) -> crate::Result<NewApplymentResponse> {
    let path = match by {
        FindApplymentBy::CustomId(id) => format!("/v3/ecommerce/applyments/out-request-no/{id}"),
        FindApplymentBy::Id(id) => format!("/v3/ecommerce/applyments/{id}"),
    };

    http::get(conf, &path, None).await
}

pub async fn change_settlement_account(
    conf: &WechatConfig,
    sub_mch_id: String,
    params: &ChangeSettlementAccountParams,
) -> crate::Result<ChangeSettlementAccountResponse> {
    http::upsert(
        conf,
        Method::POST,
        &format!("/v3/apply4sub/sub_merchants/{sub_mch_id}/modify-settlement"),
        params,
    )
    .await
}

pub async fn find_settlement_acount(
    conf: &WechatConfig,
    sub_mch_id: &str,
    application_no: &str,
) -> Result<SettlementAccount> {
    http::get(
        conf,
        &format!("/v3/apply4sub/sub_merchants/{sub_mch_id}/application/{application_no}"),
        None,
    )
    .await
}
pub async fn find_settlement_account_progress(
    conf: &WechatConfig,
    sub_mch_id: &str,
    application_no: &str,
) -> Result<FindSettlementAccountProgresss> {
    http::get(
        conf,
        &format!("/v3/apply4sub/sub_merchants/{sub_mch_id}/application/{application_no}"),
        None,
    )
    .await
}
