use serde::{Deserialize, Serialize};

use crate::alipay::api::types::contact::AliMerchantContact;

use super::{
    settle::{DefualtSettleRule, SettleCardInfo},
    site::SiteInfo,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessAddress {
    pub city_code: String,
    pub district_code: String,
    pub address: String,
    pub province: String,
    pub poiid: Option<String>,
    pub longitude: Option<String>,
    pub latitude: Option<String>,
    // #[serde(rename = "type")]
    // pub kind: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyResponse {
    // 申请单的 id
    pub order_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryQualification {
    pub industry_qualification_type: String,
    pub industry_qualification_image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlipayApplyment {
    /// 签约支付宝账户。
    /// 需使用实名认证支付宝账号，使用该支付宝账号签约直付通二级商户及后续服务，商户主体与该支付宝账号主体相同
    /// ### Example:
    /// `asdf@163.com`
    pub binding_alipay_logon_id: String,
    pub external_id: String,
    pub alias_name: String,
    pub name: String,
    pub contact_infos: AliMerchantContact,
    pub default_settle_rule: DefualtSettleRule,
    pub biz_cards: Option<SettleCardInfo>,
    pub license_auth_letter_image: Option<String>,
    pub service: Vec<String>,
    pub business_address: Option<BusinessAddress>,
    pub out_door_images: Option<String>,
    pub in_door_images: Option<String>,
    pub sites: Option<SiteInfo>,

    pub mcc: String,
    pub qualifications: Option<Vec<IndustryQualification>>,

    pub additional_cert_no: Option<String>,
    pub additional_cert_type: Option<String>,
    pub additional_cert_image: Option<String>,

    pub oversea_settle_open_id: Option<String>,
    pub oversea_settle_account: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZftSubMerchantOrder {
    pub order_id: String,
    pub external_id: String,
    pub merchant_name: Option<String>,
    pub status: Option<String>,
    pub apply_time: String,
    pub fk_audit: Option<String>,
    pub fk_audit_memo: Option<String>,
    pub kz_audit: Option<String>,
    pub kz_audit_memo: Option<String>,
    pub sub_confirm: Option<String>,
    pub card_alias_no: Option<String>,
    pub smid: Option<String>,
    pub apply_type: String,
    pub app_pre_auth: Option<bool>,
    pub face_pre_auth: Option<bool>,
    pub is_face_limit: Option<bool>,
    pub reason: Option<String>,
    pub sub_sign_qr_code_url: Option<String>,
    pub sub_sign_short_chain_url: Option<String>,
}
