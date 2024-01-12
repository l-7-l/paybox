use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct ChangeSettlementAccountParams {
    /// 修改模式。2023年4月17日前，该参数为选填；2023年4月17日（含当日）之后，无论是否传入该参数，均按照受理模式执行。
    ///
    /// 可选取值：
    ///
    ///     * MODIFY_MODE_ASYNC: 受理模式
    pub modify_mode: Option<String>,

    /// 根据特约商户号/二级商户号的主体类型，可选择的账户类型如下：
    ///
    /// 1、小微主体：经营者个人银行卡
    /// 2、个体工商户主体：经营者个人银行卡/ 对公银行账户
    /// 3、企业主体：对公银行账户
    /// 4、党政、机关及事业单位主体：对公银行账户
    /// 5、其他组织主体：对公银行账户
    ///
    /// 可选取值：
    ///
    ///     * ACCOUNT_TYPE_BUSINESS: 对公银行账户
    ///     * ACCOUNT_TYPE_PRIVATE: 经营者个人银行卡
    pub account_type: String,

    /// 请填写开户银行名称。
    ///
    /// 对私银行调用：查询支持个人业务的银行列表API
    /// 对公银行调用：查询支持对公业务的银行列表API
    pub account_bank: String,

    /// 需至少精确到市，详细参见省市区编号对照表。
    pub bank_address_code: String,

    /// 1、根据开户银行查询接口中的“是否需要填写支行”判断是否需要填写。如为其他银行，开户银行全称（含支行）和开户银行联行号二选一。
    /// 2、详细需调用查询支行列表API查看查询结果。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,

    /// 1、根据开户银行查询接口中的“是否需要填写支行”判断是否需要填写。如为其他银行，开户银行全称（含支行）和开户银行联行号二选一。
    /// 2、详细需调用查询支行列表API查看查询结果。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_branch_id: Option<String>,

    /// 1、数字，长度遵循系统支持的开户银行对照表中对公/对私卡号长度要求
    /// 2、该字段需进行加密处理，加密方法详见敏感信息加密说明。(提醒：必须在HTTP头中上送Wechatpay-Serial)
    pub account_number: String,

    /// 1、不需要修改开户名称时，可以不填写或填写当前绑定的结算银行卡户名；
    /// 2、支持将开户名称修改为当前商户对应的主体名称（对公银行账户）或经营者名称（个人银行账户），支持修改开户名称中括号的全半角；
    /// 3、该字段需进行加密处理，加密方法详见敏感信息加密说明。(提醒：必须在HTTP头中上送Wechatpay-Serial)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChangeSettlementAccountResponse {
    ///【修改结算账户申请单号】 提交二级商户修改结算账户申请后，由微信支付返回的单号，作为查询申请状态的唯一标识。
    pub application_no: String,
}

// #[derive(Debug, Clone, Serialize)]
// pub enum ModifyMode {
//     /// 受理模式
//     Async,
// }

// #[derive(Debug, Clone, Serialize)]
// pub enum AccountType {
//     /// 对公银行账户
//     Business,
//     /// 经营者个人银行卡
//     Private,
// }

#[derive(Debug, Clone, Deserialize)]
pub struct SettlementAccount {
    /// 返回特约商户的结算账户类型。
    ///
    /// 可选取值：
    ///
    ///     * ACCOUNT_TYPE_BUSINESS: 对公银行账户
    ///     * ACCOUNT_TYPE_PRIVATE: 经营者个人银行卡
    pub account_type: AccountType,

    /// 返回特约商户的结算账户-开户银行全称。
    pub account_bank: String,

    /// 返回特约商户的结算账户-开户银行全称（含支行）。
    pub bank_name: Option<String>,

    /// 返回特约商户的结算账户-联行号。
    pub bank_branch_id: Option<String>,

    /// 返回特约商户的结算账户-银行账号，掩码显示。
    pub account_number: String,

    /// 返回特约商户的结算账户-验证结果。
    ///
    /// 可选取值：
    ///
    ///     * VERIFY_SUCCESS: 验证成功，该账户可正常发起提现。
    ///     * VERIFY_FAIL: 验证失败，该账户无法发起提现，请检查修改。
    ///     * VERIFYING: 验证中，商户可发起提现尝试。
    pub verify_result: VerifyStatus,

    /// 如果验证成功则为空，验证失败则为具体原因。
    pub verify_fail_reason: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum AccountType {
    /// 对公银行账户
    #[serde(rename = "ACCOUNT_TYPE_BUSINESS")]
    Business,
    /// 经营者个人银行卡
    #[serde(rename = "ACCOUNT_TYPE_PRIVATE")]
    Private,
}

// 真尼玛卧槽
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerifyStatus {
    /// 验证成功，该账户可正常发起提现。
    VerifySuccess,
    /// 验证失败，该账户无法发起提现，请检查修改。
    VerifyFail,
    /// 验证中，商户可发起提现尝试。
    Verifying,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuditStatus {
    /// 审核成功
    AuditSuccess,
    /// 审核中
    Auditing,
    /// 审核失败
    AuditFail,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FindSettlementAccountProgresss {
    /// 开户名称，掩码显示。
    pub account_name: String,

    /// 结算账户类型。
    ///
    /// 可选取值：
    ///
    ///     * ACCOUNT_TYPE_BUSINESS: 对公银行账户
    ///     * ACCOUNT_TYPE_PRIVATE: 经营者个人银行卡
    pub account_type: AccountType,

    /// 开户银行全称。
    pub account_bank: String,

    /// 开户银行全称（含支行）。
    pub bank_name: Option<String>,

    /// 开户银行联行号。
    pub bank_branch_id: Option<String>,

    /// 银行账号，掩码显示。
    pub account_number: String,

    /// 审核状态。
    ///
    /// 可选取值：
    ///
    ///     * AUDIT_SUCCESS: 审核成功
    ///     * AUDITING: 审核中
    ///     * AUDIT_FAIL: 审核驳回
    pub verify_result: AuditStatus,

    /// 审核驳回原因。审核成功时为空，审核驳回时为具体原因。
    pub verify_fail_reason: Option<String>,

    /// 审核结果更新时间。
    ///
    /// 遵循rfc3339标准格式，格式为yyyy-MM-DDTHH:mm:ss+TIMEZONE，yyyy-MM-DD表示年月日，T出现在字符串中，表示time元素的开头，HH:mm:ss表示时分秒，TIMEZONE表示时区（+08:00表示东八区时间，领先UTC 8小时，即北京时间）。例如：2015-05-20T13:29:35+08:00表示，北京时间2015年5月20日13点29分35秒。
    pub verify_finish_time: Option<String>,
}
