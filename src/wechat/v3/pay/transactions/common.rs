use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "derive_new", derive(derive_new::new))]
pub struct StoreInfo {
    pub id: String,
    pub name: Option<String>,
    pub area_code: Option<String>,
    pub address: Option<String>,
}

/// 场景信息 支付场景信息描述
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SalesSceneInfo {
    ///【用户终端IP】 用户的客户端IP，支持IPv4和IPv6两种格式的IP地址。
    pub payer_client_ip: String,
    ///【商户端设备号】 商户端设备号（门店号或收银设备ID）
    pub device_id: Option<String>,
    ///【商户门店信息】 商户门店信息
    pub store_info: Option<StoreInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementInfo {
    pub settlement_id: Option<i32>,
    pub qualification_type: Option<String>,

    // 【是否指定分账】 是否指定分账，
    // pub profit_sharing: bool,
    // 【补差金额】 SettleInfo.profit_sharing为true时，该金额才生效。
    //
    //  单笔订单最高补差金额为5000元。
    //  仅用于合单支付!!!
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub subsidy_amount: Option<u64>,
}
