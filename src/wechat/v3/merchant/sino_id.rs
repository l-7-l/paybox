use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(into = "String")]
pub enum SinoIdKind {
    /// 中国大陆居民-身份证
    MainlandIdCard,
    /// 其他国家或地区居民-护照
    OverseaPassport,
    /// 中国香港居民--来往内地通行证
    Hongkong,
    /// 中国澳门居民--来往内地通行证
    Maco,
    /// 中国台湾居民--来往大陆通行证
    Taiwan,
    /// 外国人居留证
    ForeignerResident,
    /// 港澳居民证
    HongkongMacaoResident,
    /// 台湾居民证
    TaiwanResident,
}

impl SinoIdKind {
    pub fn as_str(&self) -> &str {
        use SinoIdKind::*;
        match self {
            MainlandIdCard => "IDENTIFICATION_TYPE_MANLAND_IDCARD",
            OverseaPassport => "IDENTIFICATION_TYPE_OVERSEA_PASSPORT",
            Hongkong => "IDENTIFICATION_TYPE_HONKONG",
            Maco => "IDENTIFICATION_TYPE_MACO",
            Taiwan => "IDENTIFICATION_TYPE_TAIWAN",
            ForeignerResident => "IDENTIFICATION_TYPE_FOREIGN_RESIDENT",
            HongkongMacaoResident => "IDENTIFICATION_TYPE_MACRO_RESIDENT",
            TaiwanResident => "IDENTIFICATION_TYPE_TIWAN_RESIDENT",
        }
    }
}

use std::fmt;
impl fmt::Display for SinoIdKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl From<SinoIdKind> for String {
    fn from(value: SinoIdKind) -> Self {
        value.as_str().to_string()
    }
}
