mod status;

pub use status::TransactionStatus;

use crate::wechat::pay::{PayerInfo, PromotionScope, WechatPayCurrency, WechatPayMethod};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Payer {
    pub sp_openid: Option<String>,
    pub sub_openid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WechatPayAmount {
    pub total: Option<u64>,
    pub payer_total: Option<u64>,
    pub currency: Option<WechatPayCurrency>,
    pub payer_currency: Option<WechatPayCurrency>,
}

// 【优惠类型】
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PromotionKind {
    /// 充值
    Cash,
    /// 预充值
    NoCash,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GoodsDetailInPromotion {
    pub good_id: String,
    pub quantity: u64,
    pub unit_price: u64,
    pub discount_amount: u64,
    pub goods_remak: Option<String>,
}

/// 优惠功能
#[derive(Debug, Clone, Deserialize)]
pub struct PromotionDetail {
    pub coupon_id: String,
    pub name: Option<String>,
    pub scope: Option<PromotionScope>,
    #[serde(rename = "type")]
    pub kind: Option<PromotionKind>,
    pub amount: u64,
    /// 活动 id
    pub stock_id: Option<String>,
    /// 微信出资
    pub wechatpay_contribute: Option<u64>,
    /// 商户出资
    pub merchant_contribute: Option<u64>,
    /// 其他出资
    pub other_contribute: Option<u64>,
    pub currency: Option<WechatPayCurrency>,
    /// 商品列表
    pub goods_detail: Option<Vec<GoodsDetailInPromotion>>,
}

/// 场景信息
#[derive(Debug, Clone, Deserialize)]
pub struct TransactionSceneInfo {
    /// 商户端设备号
    pub device_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Transaction {
    /// 应用ID
    #[cfg_attr(feature = "platform", serde(rename = "sp_appid"))]
    pub appid: Option<String>,
    /// 直连商户号
    #[cfg_attr(feature = "platform", serde(rename = "sp_mchid"))]
    pub mchid: String,

    #[cfg(feature = "platform")]
    pub sub_appid: Option<String>,
    #[cfg(feature = "platform")]
    pub sub_mchid: String,
    /// 商户订单号
    pub out_trade_no: String,
    /// 微信支付单号
    pub transaction_id: Option<String>,
    /// 微信支付方式
    pub trade_type: Option<WechatPayMethod>,
    /// 交易状态
    pub trade_state: TransactionStatus,
    ///  交易状态描述
    pub trade_state_desc: String,
    /// 付款银行
    pub bank_type: Option<String>,
    /// 附加数据
    pub attach: Option<String>,
    /// 【支付完成时间】 支付完成时间，遵循rfc3339标准格式，格式为yyyy-MM-DDTHH:mm:ss+TIMEZONE，yyyy-MM-DD表示年月日，T出现在字符串中，表示time元素的开头，HH:mm:ss表示时分秒，TIMEZONE表示时区（+08:00表示东八区时间，领先UTC 8小时，即北京时间）。例如：2015-05-20T13:29:35+08:00表示，北京时间2015年5月20日13点29分35秒。
    pub success_time: Option<String>,
    /// 支付者
    pub payer: Payer,
    /// 订单金额
    pub amount: WechatPayAmount,
    pub scene_info: Option<TransactionSceneInfo>,
    pub promotion_detail: Option<Vec<PromotionDetail>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CombineSuborderAmount {
    pub total_amount: u64,
    pub payer_amount: u64,
    pub currency: WechatPayCurrency,
    pub payer_currency: WechatPayCurrency,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CombineSuborder {
    pub mchid: String,
    pub trade_type: WechatPayMethod,
    pub trade_state: TransactionStatus,
    pub bank_type: Option<String>,
    pub attach: Option<String>,
    pub success_time: Option<String>,
    pub amount: Option<CombineSuborderAmount>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CombineTransaction {
    pub combine_appid: String,
    pub combine_mchid: String,
    pub combine_payerinfo: PayerInfo,
    pub sub_orders: Vec<CombineSuborder>,
    pub scene_info: Option<TransactionSceneInfo>,
    pub combine_out_trade_no: String,
}
