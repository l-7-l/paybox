use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IdCardInfo {
    /// 身份证人像面照片
    pub id_card_copy: String,
    /// 身份证国徽面照片
    pub id_card_national: String,
    /// 身份证姓名
    pub id_card_name: String,
    /// 身份证号码
    pub id_card_number: String,
    /// 身份证居住地址
    pub id_card_address: Option<String>,
    /// 身份证开始时间
    pub id_card_valid_time_begin: String,
    /// 身份证结束时间
    pub id_card_valid_time: String,
}
