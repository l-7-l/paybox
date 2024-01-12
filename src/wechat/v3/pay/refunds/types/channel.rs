use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 【退款渠道】 退款渠道
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RefundChannel {
    /// 原路退款
    Original,
    /// 退回到余额
    Balance,
    /// 原账户异常退到其他余额账户
    OtherBalance,
    /// 原银行卡异常退到其他银行卡
    OtherBankcard,
}
