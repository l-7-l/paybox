use serde::{Deserialize, Serialize};

use crate::wechat::pay::transactions::{ContactInfo, SettlementInfo};

use super::{account::WechatAccountInfo, IdCardInfo, IdDocInfo, SalesSceneInfo, UBO};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CertificateType {
    /// 事业单位法人证书 当主体为“事业单位”时选择此枚举值
    CERTIFICATE_TYPE_2388,

    /// 统一社会信用代码证书；当主体为“政府机关”或者“社会组织”时选择此枚举值

    /// 社会团体法人登记证书；当主体为“社会组织”时可选择此枚举值
    CERTIFICATE_TYPE_2394,

    /// 民办非企业单位登记证书；当主体为“社会组织”时可选择此枚举值
    CERTIFICATE_TYPE_2395,

    /// 基金会法人登记证书；当主体为“社会组织”时可选择此枚举值
    CERTIFICATE_TYPE_2396,
    /// 宗教活动场所登记证；当主体为“社会组织”时可选择此枚举值
    CERTIFICATE_TYPE_2399,
    /// 政府部门下发的其他有效证明文件；当主体为“社会组织”时可选择此枚举值
    CERTIFICATE_TYPE_2400,
    /// 执业许可证/执业证；当主体为“社会组织”时可选择此枚举值
    CERTIFICATE_TYPE_2520,
    /// 基层群众性自治组织特别法人统一社会信用代码证；当主体为“社会组织”时可选择此枚举值
    CERTIFICATE_TYPE_2521,
    /// 农村集体经济组织登记证；当主体为“社会组织”时可选择此枚举值
    CERTIFICATE_TYPE_2522,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WechatBusinessLicense {
    // 【证书类型】
    // 1、主体为“政府机关/事业单位/社会组织”时，请上传登记证书类型。
    // 2、主体为“个体工商户/企业”时，不填。
    // 可选取值：
    pub cert_type: Option<CertificateType>,
    /// 【营业执照扫描件】
    /// 1、主体为“个体工商户/企业”时，请上传营业执照的证件图片。
    /// 2、主体为“政府机关/事业单位/社会组织”时，请上传登记证书的证件图片。
    /// 3、可上传1张图片，请填写通过图片上传API接口预先上传图片生成好的MediaID 。
    /// 4、图片要求：
    /// （1）请上传证件的彩色扫描件或彩色数码拍摄件，黑白复印件需加盖公章（公章信息需完整） 。
    /// （2）不得添加无关水印（非微信支付商户申请用途的其他水印）。
    /// （3）需提供证件的正面拍摄件，完整、照面信息清晰可见。信息不清晰、扭曲、压缩变形、反光、不完整均不接受。
    /// （4）不接受二次剪裁、翻拍、PS的证件照片。
    pub business_license_copy: Option<String>,
    /// 【营业执照注册号】
    /// 1、主体为“个体工商户”时，请填写营业执照上的注册号/统一社会信用代码，格式需满足以下任一条件：
    /// -15位数字
    /// -18位阿拉伯数字或大写英文字母（不得包含英文字母I/O/Z/S/V），并且以9开头
    /// 2、主体为“企业”时，请填写营业执照上的注册号/统一社会信用代码，格式如下：
    /// -18位阿拉伯数字或大写英文字母（不得包含英文字母I/O/Z/S/V），并且以9开头
    pub business_license_number: String,
    /// 【商户名称】 请填写登记证书上的商户名称
    /// 1、长度为2-128个字符
    /// 2、前后不能有空格、制表符、换行符
    /// 3、不能仅含数字、特殊字符
    /// 4、仅能填写数字、英文字母、汉字及特殊字符
    /// 5、仅支持utf-8格式
    /// 6、个体户证件为以下情况时，按照个体户XXX命名（XXX是营业执照经营人姓名）：营业执照登记名称为空、仅含数字、仅含特殊字符、“无”、“无字号”
    /// 7、个体户不能使用“企业”“公司”或“农民专业合作社”结尾
    pub merchant_name: String,
    ///【经营者/法定代表人姓名】 请填写证件的经营者/法定代表人姓名
    /// 1、长度为2-100个字符
    /// 2、前后不能有空格、制表符、换行符
    /// 3、不能仅含特殊字符
    /// 4、仅能填写数字、英文字母、汉字及特殊字符
    pub legal_person: String,
    /// 【注册地址】
    /// 1、 主体为“政府机关/事业单位/社会组织”时必填，请填写登记证书的注册地址。
    /// 2、主体为“企业/个体户”时建议填写营业执照的注册地址，若该字段未填写，系统将会查询国家工商信息填入。需注意若工商信息查询不到，则会被审核驳回。
    /// 3、长度为4-128个字符
    /// 4、前后不能有空格、制表符、换行符
    /// 5、不能仅含数字、特殊字符
    /// 6、仅能填写数字、英文字母、汉字及特殊字符
    /// 7、仅支持utf-8格式
    pub company_address: Option<String>,
    pub business_time: Option<String>,
}

#[derive(Debug, Serialize)]
// #[cfg_attr(feature = "derive_new", derive(derive_new::new))]
pub struct NewSubMerchantApplyment {
    ///【业务申请编号 】 1、服务商自定义的商户唯一编号。
    /// 2、每个编号对应一个申请单，每个申请单审核通过后会生成一个微信支付商户号。
    /// 3、若申请单被驳回，可填写相同的“业务申请编号”，即可覆盖修改原申请单信息 。
    pub out_request_no: String,
    // 【主体类型】 非小微的主体类型需与营业执照/登记证书上一致，可参考选择主体指引，枚举值如下。
    // 2401：小微商户，指无营业执照的个人商家。
    // 2500：个人卖家，指无营业执照，已持续从事电子商务经营活动满6个月，且期间经营收入累计超过20万元的个人商家。（若选择该主体，请在“补充说明”填写相关描述）。
    // 4：个体工商户，营业执照上的主体类型一般为个体户、个体工商户、个体经营。
    // 2：企业，营业执照上的主体类型一般为有限公司、有限责任公司。
    // 3：事业单位，包括国内各类事业单位，如：医疗、教育、学校等单位。
    // 2502：政府机关，包括各级、各类政府机关，如机关党委、税务、民政、人社、工商、商务、市监等。
    // 1708：社会组织，包括社会团体、民办非企业、基金会、基层群众性自治组织、农村集体经济组织等组织。
    pub organization_type: String,

    ///【是否金融机构】 选填，请根据申请主体的实际情况填写，可参考选择金融机构指引：
    /// 1、若商户主体是金融机构，则填写：true。
    /// 2、若商户主体不是金融机构，则填写：false。
    // 若未传入将默认填写：false。
    #[serde(default)]
    pub finance_institution: bool,

    /// 【营业执照信息】
    ///  1、主体为“小微/个人卖家”时，不填。
    /// 2、主体为“个体工商户/企业”时，请上传营业执照。
    /// 3、主体为“政府机关/事业单位/社会组织”时，请上传登记证书。
    pub business_license_info: Option<WechatBusinessLicense>,

    //     【证件持有人类型】 1. 主体类型为政府机关/事业单位时选传：
    // （1）若上传的是法人证件，则不需要上传该字段。
    // （2）若因政策保密等原因，无法提供法人证件时，可上传经办人。 （经办人：经商户授权办理微信支付业务的人员，授权范围包括但不限于签约，入驻过程需完成账户验证）。
    // 2. 主体类型为企业/个体户/社会组织时，默认为经营者/法人，不需要上传该字段。
    // 可选取值：
    // LEGAL: 法人
    // SUPER: 经办人
    pub id_holder_type: Option<String>,
    //  【经营者/法人证件类型】
    // 1、当证件持有人类型为经营者/法人时，需要填写。其他情况，无需上传。
    // 2、主体为“小微/个人卖家”，可选择：身份证。
    // 3、主体为“个体户/企业/事业单位/社会组织”：可选择任一证件类型，主体为“政府机关”仅支持中国大陆居民-身份证类型。
    // 4、若没有填写，系统默认选择：身份证。
    // 可选取值：

    //     IDENTIFICATION_TYPE_MAINLAND_IDCARD: 中国大陆居民-身份证
    //     IDENTIFICATION_TYPE_OVERSEA_PASSPORT: 其他国家或地区居民-护照
    //     IDENTIFICATION_TYPE_HONGKONG: 中国香港居民--来往内地通行证
    //     IDENTIFICATION_TYPE_MACAO: 中国澳门居民--来往内地通行证
    //     IDENTIFICATION_TYPE_TAIWAN: 中国台湾居民--来往大陆通行证
    //     IDENTIFICATION_TYPE_FOREIGN_RESIDENT: 外国人居留证
    //     IDENTIFICATION_TYPE_HONGKONG_MACAO_RESIDENT: 港澳居民证
    //     IDENTIFICATION_TYPE_TAIWAN_RESIDENT: 台湾居民证
    pub id_doc_type: Option<String>,
    //     【法定代表人说明函】 1、当证件持有人类型为经办人时，必须上传。其他情况，无需上传。
    // 2、因政策保密等原因，无法提供法定代表人证件时，请参照示例图打印法定代表人说明函，全部信息需打印，不支持手写商户信息，并加盖公章。
    // 3、可上传1张图片，请填写通过图片上传API预先上传图片生成好的MediaID。
    pub authorize_letter_copy: Option<String>,

    pub id_card_info: Option<IdCardInfo>,

    pub id_doc_info: Option<IdDocInfo>,

    // 【经营者/法人是否为受益人】 主体类型为企业时，需要填写：
    // 1、若经营者/法人是最终受益人，则填写：true。
    // 2、若经营者/法人不是最终受益人，则填写：false。
    pub owner: Option<bool>,

    pub account_info: WechatAccountInfo,
    ///【超级管理员信息】 请填写店铺的超级管理员信息。
    /// 超级管理员需在开户后进行签约，并可接收日常重要管理信息和进行资金操作，请确定其为商户法定代表人或负责人。
    pub contact_info: ContactInfo,
    pub sales_scene_info: SalesSceneInfo,
    pub settlement_info: Option<SettlementInfo>,
    /// 【商户简称】 UTF-8格式，中文占3个字节，即最多21个汉字长度。将在支付完成页向买家展示，需与商家的实际售卖商品相符 。 嗯。。商品 ???
    pub merchant_shortname: String,

    /// 【特殊资质】 1、根据商户经营业务要求提供相关资质，详情查看《行业对应特殊资质》。
    /// 2、请提供为“申请商家主体”所属的特殊资质，可授权使用总公司/分公司的特殊资 质；
    /// 3、最多可上传5张照片，请填写通过图片上传API预先上传图片生成好的MediaID 。
    pub qualifications: Option<String>,
    /// 【补充材料】 根据实际审核情况，额外要求提供。最多可上传5张照片，请填写通过图片上传API预先上传图片生成好的MediaID
    pub business_addition_pics: Option<String>,
    pub business_adition_desc: Option<String>,
    pub ubo_info_list: Option<Vec<UBO>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewApplymentResponse {
    pub applyment_id: i64,
    pub out_request_no: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApplymentResponse {
    /// 申请状态
    /// 枚举值：
    /// CHECKING：资料校验中
    /// AUTHORIZING：待商家授权
    /// ACCOUNT_NEED_VERIFY：待账户验证
    /// AUDITING：审核中
    /// REJECTED：已驳回
    /// NEED_SIGN：待签约
    /// FINISH：完成
    /// FROZEN：已冻结
    /// CANCELED：已作废
    pub applyment_state: String,

    /// 申请状态描述
    /// 申请状态描述, 示例："审核中"
    pub applyment_state_desc: String,

    /// 签约链接
    /// 1、当申请状态为NEED_SIGN时才返回。
    /// 2、建议将链接转为二维码展示，需让申请单-管理者用微信扫码打开，完成签约。
    pub sign_url: Option<String>,

    /// 电商平台二级商户号
    /// 当申请状态为NEED_SIGN或FINISH时才返回。
    pub sub_mchid: Option<String>,

    /// 汇款账户验证信息
    /// 当申请状态为ACCOUNT_NEED_VERIFY 时有返回，可根据指引汇款，完成账户验证
    pub account_validation: Option<UpwardBankValidate>,

    /// 驳回原因详情
    /// 各项资料的审核情况。当申请状态为REJECTED或 FROZEN时才返回。
    pub audit_detail: Option<Vec<AuditDetail>>,

    /// 法人验证链接
    /// 1、当申请状态为
    /// ACCOUNT_NEED_VERIFY，且通过系统校验的申请单，将返回链接。
    /// 2、建议将链接转为二维码展示，让商户法人用微信扫码打开，完成账户验证。
    pub legal_validation_url: Option<String>,

    ///【业务申请编号】 提交接口填写的业务申请编号
    pub out_request_no: String,

    /// 业务申请编号
    /// 提交接口填写的业务申请编号
    pub applyment_id: i64,

    /// 签约状态
    /// 1、未签约-UNSIGNED：该状态下，电商平台可查询获取签约链接，引导二级商户的超级管理员完成签约；
    /// 2、已签约-SIGNED ：指二级商户的超级管理员已完成签约。注意：若申请单被驳回，商户修改了商户主体名称、法人名称、超级管理员信息、主体类型等信息，则需重新签约。
    /// 3、不可签约-NOT_SIGNABLE：该状态下，暂不支持超级管理员签约。一般为申请单处于已驳回、已冻结、机器校验中状态，无法签约。
    pub sign_state: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpwardBankValidate {
    /// 银行名称
    pub account_name: String,

    /// 银行账号
    pub account_no: Option<String>,

    /// 需要汇款的金额
    pub pay_amount: u64,

    /// 收款账户的卡号
    pub destination_account_number: String,

    /// 收款账户名
    pub destination_account_name: String,

    /// 开户行
    pub destination_account_bank: String,

    /// 开户省市
    pub city: String,
    /// 备注信息, 汇款时 需要填写的备注信息
    pub remark: String,

    // 请在此时间前完成汇款
    pub deadline: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuditDetail {
    /// 【参数名称】 提交申请单的资料项名称。
    pub param_name: String,
    /// 【驳回原因】 提交资料项被驳回原因。
    pub reject_reason: String,
}
