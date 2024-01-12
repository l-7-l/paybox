#[cfg(feature = "alipay_aes")]
mod aes_key;

#[cfg(feature = "alipay_aes")]
pub use aes_key::AesKey;

use rsa::{RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, Deserialize)]
pub struct AliPubKey {
    #[serde(deserialize_with = "pubkey_from_string")]
    pub key: RsaPublicKey,

    #[cfg(feature = "alipay_cert")]
    /// 支付宝根证书
    pub root_cert_sn: String,

    #[cfg(feature = "alipay_cert")]
    /// 应用公钥证书sn
    pub app_cert_sn: String,
}

impl AliPubKey {
    pub fn new(
        pub_pem: String,
        #[cfg(feature = "alipay_cert")] root_cert_sn: String,
        #[cfg(feature = "alipay_cert")] app_cert_sn: String,
    ) -> Self {
        use rsa::pkcs8::DecodePublicKey;

        Self {
            key: RsaPublicKey::from_public_key_pem(&pub_pem).unwrap(),
            #[cfg(feature = "alipay_cert")]
            root_cert_sn,
            #[cfg(feature = "alipay_cert")]
            app_cert_sn,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlipayKeys {
    /** 应用私钥字符串。RSA签名验签工具：https://docs.open.alipay.com/291/106097）*/
    #[serde(deserialize_with = "privkey_from_string")]
    pub private: RsaPrivateKey,
    pub public: Option<AliPubKey>,

    // 即 encrypt_key AES密钥，调用AES加解密相关接口时需要
    // aes 不是强制的公司想做就做不想做可以不做
    #[cfg(feature = "alipay_aes")]
    pub aes: AesKey,
    // 在三方业务场景中，第三方应用代商家应用调用支付宝 OPEN API 时，必须传入 app_auth_token。
    #[cfg(feature = "third_party")]
    pub app_auth_token: String,
}
impl AlipayKeys {
    pub fn new(
        priv_pem: &str,
        public: Option<AliPubKey>,

        #[cfg(feature = "alipay_aes")] aes_key: String,
        #[cfg(feature = "third_party")] app_auth_token: String,
    ) -> AlipayKeys {
        use rsa::pkcs8::DecodePrivateKey;

        Self {
            private: RsaPrivateKey::from_pkcs8_pem(priv_pem).unwrap(),
            public,

            #[cfg(feature = "alipay_aes")]
            aes: AesKey::new(aes_key).unwrap(),

            // 在三方业务场景中，第三方应用代商家应用调用支付宝 OPEN API 时，必须传入 app_auth_token。
            #[cfg(feature = "third_party")]
            app_auth_token,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlipayConfig {
    pub keys: AlipayKeys,
    pub gateway: String,
    pub app_id: String,

    /// 支付宝服务器主动通知商户服务器里指定的页面http/https路径。
    pub notify_url: Option<String>,
    /// app接入场景不设置，设置不生效，同步返回通过客户端接入跳转。
    /// H5场景接入设置必传，用户支付后页面同步跳转，若 return_url 设置在 alipay.trade.wap.merge.pay（无线 Wap 合并支付接口 2.0） 不生效。
    pub return_url: Option<String>,
    /// 服务器地址
    pub ws_service_url: Option<String>,
    pub timeout: u64,
    pub default_rate: u8,
}
impl AlipayConfig {
    pub fn new(
        keys: AlipayKeys,
        gateway: String,
        app_id: String,
        default_rate: u8,
        notify_url: Option<String>,
        return_url: Option<String>,
    ) -> Self {
        Self {
            keys,
            gateway,
            app_id,
            notify_url,
            return_url,
            ws_service_url: None,
            timeout: 5000,
            default_rate,
        }
    }
}

pub fn privkey_from_string<'de, D>(deserializer: D) -> Result<RsaPrivateKey, D::Error>
where
    D: Deserializer<'de>,
{
    use rsa::pkcs8::DecodePrivateKey;

    String::deserialize(deserializer).map(|pem| RsaPrivateKey::from_pkcs8_pem(&pem).unwrap())
}

pub fn pubkey_from_string<'de, D>(deserializer: D) -> Result<RsaPublicKey, D::Error>
where
    D: Deserializer<'de>,
{
    use rsa::pkcs8::DecodePublicKey;

    String::deserialize(deserializer).map(|pem| RsaPublicKey::from_public_key_pem(&pem).unwrap())
}
