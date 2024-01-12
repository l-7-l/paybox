use serde::{Deserialize, Serialize};

// 怎么个事?
///【出资账户类型】 出资账户类型
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RefundFromAccount {
    // 可用余额
    Available,

    // 不可用余额
    Unavailable,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RefundFrom {
    pub account: RefundFromAccount,
    pub amount: u64,
}
