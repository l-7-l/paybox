use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdDocInfo {
    /// 证件姓名
    pub id_doc_name: String,
    /// 证件号码
    pub id_doc_number: String,
    /// 证件正面照片的 MediaID
    pub id_doc_copy: String,
    /// 证件反面照片的 MediaID
    pub id_doc_copy_back: Option<String>,
    /// 证件居住地址
    pub id_doc_address: Option<String>,
    /// 证件开始日期
    pub doc_period_begin: String,
    /// 证件结束日期
    pub doc_period_end: String,
}
