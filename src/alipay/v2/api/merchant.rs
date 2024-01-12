pub mod applyment;
pub mod qualification;
pub mod settle;
pub mod site;

pub use applyment::ApplyResponse;
use serde::Serialize;

use crate::{
    alipay::{config::AlipayConfig, client},
    Result,
};

use applyment::{AlipayApplyment, ZftSubMerchantOrder};

pub async fn new_apply(conf: &AlipayConfig, params: &AlipayApplyment) -> Result<ApplyResponse> {
    client::request(
        "ant.merchant.expand.indirect.zft.simplecreate",
        conf,
        params,
        false,
    )
    .await
}

pub async fn get_apply_orders<P: Serialize>(
    conf: &AlipayConfig,
    params: P,
) -> Result<Vec<ZftSubMerchantOrder>> {
    client::request(
        "ant.merchant.expand.indirect.zftorder.query",
        conf,
        params,
        false,
    )
    .await
}
