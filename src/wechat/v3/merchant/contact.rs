use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::SinoIdKind;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Admin {
    /// 【超级管理员类型】
    /// 1、主体为“小微/个人卖家 ”，可选择：65-经营者/法人。
    /// 2、主体为“个体工商户/企业/政府机关/事业单位/社会组织”，可选择：65-经营者/法人、66- 经办人。 （经办人：经商户授权办理微信支付业务的人员）。
    contact_type: ContactType,
    /// 【超级管理员姓名】
    /// 1、若管理员类型为“法人”，则该姓名需与法人身份证姓名一致。
    /// 2、若管理员类型为“经办人”，则可填写实际经办人的姓名。
    /// 3、该字段需进行加密处理，加密方法详见敏感信息加密说明。(提醒：必须在HTTP头中上送Wechatpay-Serial)
    /// （后续该管理员需使用实名微信号完成签约）
    contact_name: Option<String>,
    /// 【超级管理员证件类型】
    /// 当超级管理员类型是经办人时，请上传超级管理员证件类型。
    contact_id_doc_type: Option<SinoIdKind>,
    /// 超级管理员证件号码
    contact_id_card_number: Option<String>,
    /// 超级管理员证件正面照片
    contact_id_doc_copy: Option<String>,
    /// 超级管理员证件反面照片
    contact_id_doc_copy_back: Option<String>,
    /// 超级管理员证件有效期开始时间
    contact_id_doc_period_begin: Option<String>,
    /// 超级管理员证件有效期结束时间
    contact_id_doc_period_end: Option<String>,
    /// 业务办理授权函
    business_authorization_letter: Option<String>,
    /// 超级管理员手机
    mobile_phone: String,
    /// 超级管理员邮箱
    contact_email: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(i16)]
pub enum ContactType {
    /// 法定代表人
    LegalPerson = 65,
    /// 经办人
    Operator = 66,
}
