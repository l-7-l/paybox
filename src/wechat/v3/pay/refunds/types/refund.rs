use serde::Deserialize;

use crate::wechat::pay::{transactions::PromotionDetail, PromotionScope, WechatPayCurrency};

use super::{
    channel::RefundChannel, fund_account::FundsAccount, refund_from::RefundFrom,
    status::RefundStatus,
};

#[derive(Debug, Clone, Deserialize)]
pub struct RefundAmount {
    pub total: u64,
    pub refund: u64,
    pub from: Option<Vec<RefundFrom>>,
    ///【用户支付金额】 现金支付金额，单位为分，只能为整数
    pub payer_total: u64,
    ///【用户退款金额】 退款给用户的金额，单位为分，不包含所有优惠券金额
    pub payer_refund: u64,

    ///【应结退款金额】 去掉非充值代金券退款金额后的退款金额，单位为分，退款金额=申请退款金额-非充值代金券退款金额，退款金额<=申请退款金额
    pub settlement_refund: u64,
    // 【应结订单金额】 应结订单金额=订单金额-免充值代金券金额，应结订单金额<=订单金额，单位为分
    pub settlement_total: u64,

    /// 【应结订单金额】 应结订单金额=订单金额-免充值代金券金额，应结订单金额<=订单金额，单位为分
    pub discount_refund: u64,
    ///【退款币种】 符合ISO 4217标准的三位字母代码，目前只支持人民币：CNY。
    pub currency: WechatPayCurrency,
    pub refund_fee: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]

pub enum RefundPromotionKind {
    /// 代金券类型，需要走结算资金的充值型代金券
    Coupon,
    /// 优惠券类型，不走结算资金的免充值型优惠券
    Discount,
}

/// 【商品列表】 优惠商品发生退款时返回商品信息
#[derive(Debug, Clone, Deserialize)]
pub struct RefundPromotionGoodsDetail {
    pub merchant_goods_id: String,
    pub wechatpay_goods_id: Option<String>,
    pub goods_name: Option<String>,
    pub unit_price: u64,
    pub refund_amount: u64,
    pub refund_quantity: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RefundPromotion {
    /// 【券ID】 券或者立减优惠id
    pub promotion_id: String,
    pub scope: PromotionScope,
    #[serde(rename = "type")]
    pub kind: RefundPromotionKind,
    /// 【优惠券面额】 用户享受优惠的金额（优惠券面额=微信出资金额+ 商家出资金额+其他出资方金额 ），单位为分
    pub amount: u64,
    pub refund_amount: u64,
    pub goods_detail: Vec<RefundPromotionGoodsDetail>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Refund {
    /// 【微信支付退款号】 微信支付退款号
    pub refund_id: String,
    /// 【商户退款单号】 商户系统内部的退款单号，商户系统内部唯一，只能是数字、大小写字母_-|*@ ，同一退款单号多次请求只退一笔。
    pub out_refund_no: String,
    /// 【微信支付订单号】 微信支付交易订单号
    pub transaction_id: String,
    ///【商户订单号】 原支付交易对应的商户订单号
    pub out_trade_no: String,
    /**
     #### 【退款入账账户】
      取当前退款单的退款入账方，有以下几种情况：

            1. 退回银行卡：       {银行名称}{卡类型}{卡尾号}
            2. 退回支付用户零钱：  支付用户零钱
            3. 退还商户：         商户基本账户商户结算银行账户
            4. 退回支付用户零钱通：支付用户零钱通
    */
    pub channel: RefundChannel,
    pub user_received_account: String,
    /// #### 【退款成功时间】
    /// 退款状态status为SUCCESS（退款成功）时，返回该字段。
    ///
    /// 遵循rfc3339标准格式，格式为YYYY-MM-DDTHH:mm:ss+TIMEZONE，
    /// * YYYY-MM-DD表示年月日，
    /// * T出现在字符串中，表示time元素的开头，
    /// * HH:mm:ss表示时分秒，
    /// * TIMEZONE表示时区（+08:00表示东八区时间，领先UTC 8小时，即北京时间）。
    ///
    /// ##### 例如：
    /// 2015-05-20T13:29:35+08:00表示，北京时间2015年5月20日13点29分35秒。
    pub success_time: Option<String>,
    /// #### 【退款创建时间】
    /// 退款受理时间，遵循rfc3339标准格式，格式为YYYY-MM-DDTHH:mm:ss+TIMEZONE
    /// * YYYY-MM-DD表示年月日，
    /// * T出现在字符串中，表示time元素的开头，
    /// * HH:mm:ss表示时分秒，
    /// * TIMEZONE表示时区（+08:00表示东八区时间，领先UTC 8小时，即北京时间）。
    /// #### 例如：
    /// 2015-05-20T13:29:35+08:00表示，北京时间2015年5月20日13点29分35秒。
    pub create_time: String,
    ///【退款状态】
    /// 退款到银行发现用户的卡作废或者冻结了，导致原路退款银行卡失败，
    /// 可前往商户平台（pay.weixin.qq.com）-交易中心，手动处理此笔退款。
    /// 可选取值：
    ///     SUCCESS: 退款成功
    ///     CLOSED: 退款关闭
    ///     PROCESSING: 退款处理中
    ///     ABNORMAL: 退款异常
    pub status: RefundStatus,
    /// 【资金账户】 退款所使用资金对应的资金账户类型
    ///  可选取值：
    ///     UNSETTLED: 未结算资金
    ///     AVAILABLE: 可用余额
    ///     UNAVAILABLE: 不可用余额
    ///     OPERATION: 运营户
    ///     BASIC: 基本账户（含可用余额和不可用余额）
    ///     ECNY_BASIC: 数字人民币基本账户
    pub funds_account: Option<FundsAccount>,
    pub amount: RefundAmount,
    pub promotion_detail: Option<PromotionDetail>,
}
