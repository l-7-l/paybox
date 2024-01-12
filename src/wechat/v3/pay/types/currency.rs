use serde::{Deserialize, Serialize};

/// 【退款币种】 符合ISO 4217标准的三位字母代码，目前只支持人民币：CNY。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged, from = "String")]
pub enum WechatPayCurrency {
    /// 【标价币种】 符合ISO 4217标准的三位字母代码，人民币：CNY
    CNY,
}

impl WechatPayCurrency {
    pub fn as_str(&self) -> &str {
        use WechatPayCurrency::*;
        match self {
            CNY => "CNY",
        }
    }
}

impl std::fmt::Display for WechatPayCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl From<String> for WechatPayCurrency {
    fn from(_: String) -> Self {
        // 目前只支持"CNY"
        Self::CNY
    }
}
