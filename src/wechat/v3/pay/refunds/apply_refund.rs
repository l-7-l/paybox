use crate::{
    wechat::{conf::WechatConfig, http},
    Result,
};
use reqwest::Method;

mod command;

use super::types::refund::Refund;
pub use command::{ApplyRefundAmount, ApplyRefundCommand, RefundGoodsDetail};

pub async fn apply(conf: &WechatConfig, cmd: &ApplyRefundCommand) -> Result<Refund> {
    http::upsert(conf, Method::POST, "/v3/refund/domestic/refunds", cmd).await
}
