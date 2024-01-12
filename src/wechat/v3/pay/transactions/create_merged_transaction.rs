use serde::{Deserialize, Serialize};

use crate::wechat::pay::types::PayerInfo;

use super::common::{SalesSceneInfo, SettlementInfo};

#[derive(Serialize)]
pub struct CreateMergedTransactionsParams {
    /// 【合单商户Appid】 合单发起方的Appid
    pub combine_appid: String,
    /// 【合单商户订单号】 商户系统内部订单号，可以是数字、大小写字母以及特殊符号_-*的任意组合，且在同一个商户号下唯一。
    pub combine_out_trade_no: String,
    /// 【合单商户号】 合单发起方商户号
    pub combine_mch_id: String,
    ///【场景信息】 场景信息
    pub scene_info: Option<SalesSceneInfo>,
    pub sub_orders: Vec<SubOrder>,

    pub combine_payer_info: Option<PayerInfo>,
    pub time_start: String,
    pub time_expire: String,
    // 【通知地址】 接收微信支付异步通知回调地址，通知url必须为直接可访问的url，不能携带参数。必须使用https协议。
    pub notify_url: String,
    pub limit_pay: Option<Vec<String>>,
    pub contract_info: Option<ContactInfo>,
}

#[derive(Deserialize, Serialize)]
pub struct WechatMergedSubOrderAmount {
    pub total_amount: u64,
    pub currency: String,
}

///【子单信息】 子单列表，最多支持子单条数：50。
#[derive(Deserialize, Serialize)]
pub struct SubOrder {
    /// 【子单商户号】 子单发起方商户号，与发起方Appid有绑定关系
    pub mchid: String,
    ///【附加数据】 附加数据，在查询API和支付通知中原样返回，可作为自定义参数使用。
    pub attach: String,
    /// 【订单金额】 订单金额
    pub amount: WechatMergedSubOrderAmount,
    /// 【子单商户订单号】 商户系统内部订单号，最短2个字符，最长32个字符，只能是数字、大小写字母_-|* ，且在同一个商户号下唯一。
    pub out_trade_no: String,

    /// 【特约商户商户号】 特约商户商户号
    pub sub_mchid: Option<String>,
    ///【商品详情】 商品详细描述
    pub detail: Option<String>,
    ///【商品描述】 商品简单描述。需传入应用市场上的APP名字-实际商品名称，例如 天天爱消除-游戏充值
    pub description: String,
    /// 【结算信息】 结算信息
    pub settle_info: Option<SettlementInfo>,
    ///【订单优惠标记】 订单优惠标记，使用代金券或立减优惠功能时需要的参数，说明详见代金券或立减优惠
    pub goods_tag: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContactInfo {
    pub contact_type: String,
    pub contact_name: String,
    pub contact_id_doc_type: Option<String>,
    pub contact_id_card_number: Option<String>,
    pub contact_id_doc_copy: Option<String>,
    pub contact_id_doc_copy_back: Option<String>,
    pub contact_id_doc_period_begin: Option<String>,
    pub contact_id_doc_period_end: Option<String>,
    pub business_authorization_letter: Option<String>,
    pub mobile_phone: String,
    pub contact_email: Option<String>,
   // pub mchid: String,
    // pub appid: String,
    // pub out_contract_code: String,
    // ///【委托代扣协议模板ID】 已经通过审核，且生效中的委托代扣模板ID。
    // pub plan_id: i64,
    // pub contract_display_account: String,

    // ///【回调通知地址】 接收微信支付签约结果（不包括支付结果）通知回调地址，通知URL必须为直接可访问的URL，不能携带参数。
    // pub notify_url: String,
}
