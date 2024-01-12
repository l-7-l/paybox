use serde::{Deserialize, Serialize};

/// AKA  trade_type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum WechatPayMethod {
    /// APP 支付
    App,

    /// 刷脸支付
    Facepay,
    /// 公众号支付
    Jsapi,
    /// 付款码支付
    MicroPay,
    /// H5 支付
    Mweb,
    /// 扫码支付
    Native,
}

impl WechatPayMethod {
    pub fn as_str(&self) -> &str {
        use WechatPayMethod::*;
        match self {
            App => "APP",
            Facepay => "FACEPAY",
            Jsapi => "JSAPI",
            MicroPay => "MICROPAY",
            Mweb => "MWEB",
            Native => "NATIVE",
        }
    }

    pub fn channel_name(&self) -> &str {
        use WechatPayMethod::*;

        match self {
            App => "app",
            Jsapi => "jsapi",
            Mweb => "h5",
            Native => "native",

            Facepay => "facepay",
            MicroPay => "micropay",
        }
    }
}

// impl TryFrom<String> for WechatPayMethod {
//     type Error = Error;

//     fn try_from(code: String) -> Result<Self> {
//         use WechatPayMethod::*;
//         match code.as_str() {
//             "APP" => Ok(App),
//             "JSAPI" => Ok(Jsapi),
//             "MWEB" => Ok(Mweb),
//             "NATIVE" => Ok(Native),
//             "MICROPAY" => Ok(MicroPay),
//             "FACEPAY" => Ok(Facepay),
//             _ => Err(crate::Error::Invalid {
//                 line: 59,
//                 code: "invalid_wechat_pay_method".to_owned(),
//             }),
//         }
//     }
// }

use std::fmt::{Display, Formatter};

impl Display for WechatPayMethod {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
