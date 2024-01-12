use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WechatAccountInfo {
    /// 【账户类型】
    /// 1、若主体为企业/政府机关/事业单位/社会组织，可填写：74-对公账户。
    /// 2、主体为小微/个人卖家，可选择：75-对私账户。
    /// 3、若主体为个体工商户，可填写：74-对公账户、75-对私账户。
    pub bank_account_type: String,
    /// 开户银行
    pub account_bank: String,
    /// 开户名称
    /// 【开户名称】
    /// 1、选择经营者个人银行卡时，开户名称必须与身份证姓名一致。
    /// 2、选择对公账户时，开户名称必须与营业执照上的“商户名称”一致。
    /// 3、该字段需进行加密处理，加密方法详见敏感信息加密说明。(提醒：必须在HTTP头中上送Wechatpay-Serial)
    pub account_name: String,
    /// 开户银行省市编码
    pub bank_address_code: String,
    /// 开户银行联行号
    pub bank_branch_id: Option<String>,
    /// 开户银行全称（含支行）
    pub bank_name: Option<String>,
    /// 银行账号
    /// 1、数字，长度遵循系统支持的对公/对私卡号长度要求表。(https://pay.weixin.qq.com/docs/partner/development/chart/bank-name.html)
    /// 2、该字段需进行加密处理，加密方法详见敏感信息加密说明。(提醒：必须在HTTP头中上送Wechatpay-Serial)
    pub account_number: String,
}
