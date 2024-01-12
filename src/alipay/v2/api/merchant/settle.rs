use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SettleType {
    BankCard,
    AlipayAccount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefualtSettleRule {
    pub default_settle_type: SettleType,
    pub default_settle_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettleCardInfo {
    pub account_holder_name: String,
    pub account_no: String,
    pub account_inst_province: String,
    pub account_inst_city: String,
    pub account_branch_name: String,
    pub usage_type: String,
    pub account_type: String,
    pub account_inst_name: String,
    pub account_inst_id: String,
    pub bank_code: Option<String>,
}
