use serde::{Deserialize, Serialize};

/// #### 【退款状态】
///
///  退款到银行发现用户的卡作废或者冻结了，导致原路退款银行卡失败，可前往商户平台（pay.weixin.qq.com）-交易中心，
/// 手动处理此笔退款。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RefundStatus {
    Success,
    Closed,
    Processing,
    Abnormal,
}
