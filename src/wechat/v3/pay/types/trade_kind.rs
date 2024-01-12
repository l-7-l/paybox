use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TradeKind {
    Jsapi,
    Native,
    App,
    MicroPay,
    Mweb,
    Facepay,
}

impl TradeKind {
    pub fn as_str(&self) -> &str {
        match self {
            TradeKind::Jsapi => "JSAPI",
            TradeKind::Native => "NATIVE",
            TradeKind::App => "APP",
            TradeKind::MicroPay => "MICROPAY",
            TradeKind::Mweb => "MWEB",
            TradeKind::Facepay => "FACEPAY",
        }
    }
}

// impl FromStr for TradeKind {
//     type Err = Error;

//     fn from_str(code: &str) -> Result<Self, Self::Err> {
//         use TradeKind::*;
//         let kind = match code {
//             "JSAPI" => Jsapi,
//             "NATIVE" => Native,
//             "APP" => App,
//             "MICROPAY" => MicroPay,
//             "MWEB" => Mweb,
//             "FACEPAY" => Facepay,
//             _ => {
//                 return Err(Error::invalid("invalid_wechat_trade_kind"));
//             }
//         };

//         Ok(kind)
//     }
// }

// impl TryFrom<String> for TradeKind {
//     type Error = Error;

//     fn try_from(code: String) -> Result<Self, Self::Error> {
//         Self::from_str(&code)
//     }
// }

// impl From<TradeKind> for String {
//     fn from(value: TradeKind) -> Self {
//         value.to_string()
//     }
// }

// impl Display for TradeKind {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str(self.as_str())
//     }
// }
