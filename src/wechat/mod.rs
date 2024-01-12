#[cfg(feature = "wechat_pay_v3")]
mod v3;

#[cfg(feature = "wechat_pay_v3")]
pub use v3::*;
