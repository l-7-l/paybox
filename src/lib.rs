mod crypto;
mod day;
pub mod errors;
pub mod types;
mod url;

pub use errors::{Error, Result};

#[cfg(feature = "alipay")]
pub mod alipay;

#[cfg(any(
    feature = "wechat_pay_v3",
    feature = "wechat_pay_app",
    feature = "wechat_pay_h5",
    feature = "wechat_pay_jsapi",
    feature = "wechat_pay_v3"
))]
pub mod wechat;

#[derive(Debug, Clone)]
pub enum PayboxMode {
    Normal,

    ///
    /// <p style="background:rgba(255,181,77,0.16);padding:0.75em;">
    /// <strong>WARING:</strong>
    ///
    ///     微信支付不支持沙盒模式。
    ///
    ///     WeChatPay does not support sandbox mode.
    /// </p>
    ///
    Sandbox,
}
