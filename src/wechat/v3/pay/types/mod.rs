mod currency;
pub mod error_response;
mod method;
mod promotion;
pub mod trade_kind;
pub mod trade_state;

pub use currency::WechatPayCurrency;
pub use method::WechatPayMethod;
pub use promotion::PromotionScope;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PayerInfo {
    pub open_id: Option<String>,
}

// {"code":"PARAM_ERROR","detail":{"location":"body","value":" https://www.weixin.qq.com/wxpay/pay.php"},"message":"输入源“/body/notify_url”映射到值字段“通知地址”字符串规则校验失败，字符串必须匹配正则表达式“^https?://([^\\s/?#\\[\\]\\@]+\\@)?([^\\s/?#\\@:]+)(?::\\d{2,5})?([^\\s?#\\[\\]]*)$”"}l
