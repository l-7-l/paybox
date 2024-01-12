use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FundsAccount {
    /// 未结算资金
    Unsettled,
    /// 可用余额
    Available,
    /// 不可用余额
    Unavailable,
    /// 运营户
    Operation,
    /// 基本账户（含可用余额和不可用余额）
    Basic,
    /// 数字人民币基本账户
    EcnyBasic,
}
