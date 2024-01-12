use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WechatMerchantConfig {
    /// 【公众号ID】 公众号ID
    #[cfg(not(feature = "platform"))]
    appid: String,

    #[cfg(feature = "platform")]
    sp_appid: String,

    #[cfg(not(feature = "platform"))]
    mchid: String,

    #[cfg(feature = "platform")]
    sp_mchid: String,
    serialno: String,
}

impl WechatMerchantConfig {
    pub fn new(app_id: String, mch_id: String, serialno: String) -> Self {
        Self {
            #[cfg(not(feature = "platform"))]
            appid: app_id,
            #[cfg(feature = "platform")]
            sp_appid: app_id,
            #[cfg(not(feature = "platform"))]
            mchid: mch_id,
            #[cfg(feature = "platform")]
            sp_mchid: mch_id,
            serialno,
        }
    }

    pub fn app_id(&self) -> &str {
        #[cfg(not(feature = "platform"))]
        return &self.appid;

        #[cfg(feature = "platform")]
        &self.sp_appid
    }

    pub fn mch_id(&self) -> &str {
        #[cfg(not(feature = "platform"))]
        return &self.mchid;

        #[cfg(feature = "platform")]
        &self.sp_mchid
    }

    pub fn serialno(&self) -> &str {
        &self.serialno
    }
}
