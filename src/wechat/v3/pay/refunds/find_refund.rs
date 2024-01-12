use crate::{
    wechat::{conf::WechatConfig, http},
    Result,
};

use super::types::refund::Refund;

pub async fn find_by_custom_id<Id: Into<String>>(
    conf: &WechatConfig,
    id: Id,
    #[cfg(feature = "platform")] sub_mch_id: Id,
) -> Result<Refund> {
    http::get(
        conf,
        &format!("/v3/refund/domestic/refunds/{}", id.into()),
        #[cfg(feature = "platform")]
        Some(&format!("?sub_mchid={}", sub_mch_id.into())),
        #[cfg(not(feature = "platform"))]
        None,
    )
    .await
}
