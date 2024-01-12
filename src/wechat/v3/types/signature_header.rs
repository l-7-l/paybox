use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
pub struct SignatureHeader {
    /// 时间戳
    pub time_stamp: String,
    /// 随机串
    pub nonce: String,
    /// 已签名字符串
    pub signature: String,
    /// 证书序列号
    pub serial: String,
}

impl SignatureHeader {
    pub fn from_header(header: &HeaderMap) -> Self {
        let timpestamp = header.get("Wechatpay-Timestamp");
        let time_stamp = timpestamp
            .map(|h| h.to_str().unwrap_or_default().to_string())
            .unwrap_or_default();
        let nonce = header.get("Wechatpay-Nonce");
        let nonce = nonce
            .map(|h| h.to_str().unwrap_or_default().to_string())
            .unwrap_or_default();
        let signature = header.get("Wechatpay-Signature");
        let signature = signature
            .map(|h| h.to_str().unwrap_or_default().to_string())
            .unwrap_or_default();
        let serial = header.get("Wechatpay-Serial");
        let serial = serial
            .map(|h| h.to_str().unwrap_or_default().to_string())
            .unwrap_or_default();

        SignatureHeader {
            time_stamp,
            nonce,
            signature,
            serial,
        }
    }
}
