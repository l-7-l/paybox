use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatPayNotify {
    pub id: String,
    pub create_time: String,
    pub event_type: String,
    pub resource_type: String,
    pub summary: String,
    pub resource: WechatPayNotifyResource,
}

impl WechatPayNotify {
    pub fn is_success_paid(&self) -> bool {
        self.event_type.as_str() == "TRANSACTION_SUCCESS"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatPayNotifyResource {
    pub algorithm: String,
    pub ciphertext: String,
    pub associated_data: Option<String>,
    /// 非合单支付下为 None
    pub original_type: Option<String>,
    pub nonce: String,
}
