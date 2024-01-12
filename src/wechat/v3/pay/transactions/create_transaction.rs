use serde::{Deserialize, Serialize};

use crate::wechat::{conf::WechatMerchantConfig, pay::error_response::WechatError};

use super::{common::SalesSceneInfo, order_detail::OrderDetail};

#[derive(Deserialize, Serialize)]
pub struct TransactionSettlementInfo {
    pub profit_sharing: bool,
}

#[derive(Deserialize, Serialize)]
pub struct CreateTransactionParams {
    /// 【公众号ID】 公众号ID
    // #[cfg(not(feature = "platform"))]
    // pub appid: String,
    ///【直连商户号】 直连商户号
    // #[cfg(not(feature = "platform"))]
    // pub mchid: String,

    /// 【服务商公众号ID】 服务商公众号ID
    // 非必传
    // #[cfg(feature = "platform")]
    // pub sp_appid: String,
    // #[cfg(feature = "platform")]
    ///【服务商户号】 服务商户号
    // pub sp_mchid: String,

    ///【子商户/二级商户应用ID】 子商户/二级商户在开放平台申请的应用AppID。
    #[serde(flatten)]
    pub mch_conf: WechatMerchantConfig,
    // #[cfg(feature = "platform")]
    // pub sub_appid: Option<String>,
    ///【子商户号/二级商户号】 子商户/二级商户的商户号，由微信支付生成并下发。
    #[cfg(feature = "platform")]
    pub sub_mchid: String,

    /// 【商户订单号】 商户系统内部订单号，只能是数字、大小写字母_-*且在同一个商户号下唯一
    pub out_trade_no: String,

    /// 【商品描述】 商品描述
    pub description: String,

    /// 【交易结束时间】订单失效时间，遵循rfc3339标准格式
    /// 格式为 yyyy-MM-DDTHH:mm:ss+TIMEZONE
    /// yyyy-MM-DD表示年月日，
    /// T出现在字符串中，表示time元素的开头，
    /// HH:mm:ss表示时分秒，
    /// TIMEZONE表示时区（+08:00表示东八区时间，领先UTC8小时，即北京时间）。
    /// 例如：2015-05-20T13:29:35+08:00 表示，北京时间2015年5月20日13点29分35秒。
    pub time_expire: Option<String>,
    /// 【附加数据】 附加数据
    pub attach: Option<String>,

    ///【通知地址】
    /// 异步接收微信支付结果通知的回调地址，通知URL必须为外网可访问的URL，不能携带参数。
    /// 公网域名必须为HTTPS，如果是走专线接入，使用专线NAT IP或者私有回调域名可使用HTTP
    pub notify_url: String,
    /// 【订单优惠标记】 商品标记，代金券或立减优惠功能的参数。
    pub goods_tag: Option<String>,

    ///【结算信息】 结算信息
    pub settle_info: Option<TransactionSettlementInfo>,

    ///【电子发票入口开放标识】 传入true时，支付成功消息和支付详情页将出现开票入口。需要在微信支付商户平台或微信公众平台开通电子发票功能，传此字段才可生效。
    pub support_fapiao: Option<bool>,
    ///【订单金额】 订单金额
    pub amount: WechatPrePayAmount,
    ///【优惠功能】 优惠功能
    pub detail: Option<OrderDetail>,
    ///【场景信息】 场景信息
    pub scene_info: Option<SalesSceneInfo>,
}

#[derive(Deserialize, Serialize)]
pub struct WechatPrePayAmount {
    pub total: u64,
    pub currency: String,
    // WechatPayCurrency,
}

#[derive(Deserialize)]
pub struct AppPrePayResponse {
    #[serde(flatten)]
    pub base: WechatError,
    pub prepay_id: String,
}
