use super::WechatMerchantConfig;
use crate::{day, wechat::v3::crypto};

use rsa::RsaPublicKey;
use serde::Deserialize;

pub use rsa::RsaPrivateKey;

#[derive(Debug, Clone, Deserialize)]
pub struct WechatConfig {
    pub merchant: WechatMerchantConfig,
    pub private_key: RsaPrivateKey,
    pub public_key: RsaPublicKey,
    pub api_key: String,
    pub shop_base_url: String,
    pub default_rate: u8,

    /// 用于接收微信支付结果通知的回调地址
    pub notify_url: String,
    pub refund_notify_url: String,
}

impl WechatConfig {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        merchant: WechatMerchantConfig,
        priv_pem: &str,
        pub_pem: &str,
        api_key: String,
        shop_base_url: String,
        default_rate: u8,
        notify_url: String,
        refund_notify_url: String,
    ) -> crate::Result<Self> {
        use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey};

        Ok(Self {
            merchant,
            private_key: RsaPrivateKey::from_pkcs8_pem(priv_pem).unwrap(),
            public_key: RsaPublicKey::from_public_key_pem(pub_pem).unwrap(),
            api_key,
            shop_base_url,
            default_rate,
            notify_url,
            refund_notify_url,
        })
    }
    /// <p style="background:rgba(255,181,77,0.16);padding:0.75em;">
    /// <strong>Warning:</strong>
    ///     微信支付 v3 版本没有沙盒模式!!
    /// </p>
    pub(crate) fn url(&self, subpath: &str) -> String {
        format!("https://api.mch.weixin.qq.com/{subpath}")
    }

    /// 请求方法为GET时，报文主体为空。
    /// 当请求方法为POST或PUT时，请使用真实发送的JSON报文。
    /// 图片上传API，请使用meta对应的JSON报文。
    pub(crate) fn signature(
        &self,
        http_method: &str,
        url: &str,
        data: Option<&str>,
    ) -> crate::Result<String> {
        let mch_id = self.merchant.mch_id();
        let serialno = self.merchant.serialno();
        let timestamp = day::timestamp();
        let nonce = crypto::nonce();

        // 请求方法为GET时，报文主体为空。
        // 当请求方法为POST或PUT时，请使用真实发送的JSON报文。
        // 图片上传API，请使用meta对应的JSON报文。
        // let body_str = if body.is_some() && (http_method == "POST" || http_method == "PUT") {
        //     body.unwrap()
        //     // serde_json::to_string(body.unwrap())?
        // } else {
        //     ""
        // };

        let mut query = "";
        let mut body = "";

        if let Some(x) = data {
            if http_method == "GET" {
                query = x;
            } else if http_method == "POST" || http_method == "PUT" {
                body = x;
            }
        }

        let token = format!("{http_method}\n{url}{query}\n{timestamp}\n{nonce}\n{body}\n");
        let encoded_token = crypto::encode_signature(self.private_key.clone(), token.as_bytes());

        Ok(format!(
            r#"WECHATPAY2-SHA256-RSA2048 mch_id="{mch_id}",nonce_str="{nonce},timestamp={timestamp},serialno="{serialno}",signature="{encoded_token}"""#
        ))
    }
}
