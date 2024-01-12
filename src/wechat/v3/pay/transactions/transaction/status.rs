use serde::{Deserialize, Serialize};

/// 【交易状态】 交易状态，枚举值：
/// * SUCCESS：支付成功
/// * REFUND：转入退款
/// * NOTPAY：未支付
/// * CLOSED：已关闭
/// * REVOKED：已撤销（仅付款码支付会返回）
/// * USERPAYING：用户支付中（仅付款码支付会返回）
/// * PAYERROR：支付失败（仅付款码支付会返回）

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TransactionStatus {
    Success, // 支付成功
    Refund,  // 转入退款
    NotPay,  // 未支付

    Closed, // 已关闭

    Revoked, // 已撤销

    UserPaying, // 用户支付中

    PayError, // 支付失败
}
