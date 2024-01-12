use crate::wechat::pay::{
    refunds::types::refund_from::RefundFrom, transactions::GoodsDetail, WechatPayCurrency,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct RefundGoodsDetail {
    ///【商户侧商品编码】 商品编码，由半角的大小写字母、数字、中划线、下划线中的一种或几种组成。
    pub merchant_goods_id: String,
    /// 【微信侧商品编码】 微信支付定义的统一商品编号（没有可不传）
    pub wechatpay_goods_id: Option<String>,
    /// 【商品名称】 商品的实际名称
    pub goods_name: Option<String>,
    /// 【商品单价】 商品单价金额，单位为分
    pub unit_price: u64,
    /// 【商品退款金额】 商品退款金额，单位为分
    pub refund_amount: u64,
    /// 【商品退货数量】 对应商品的退货数量
    pub refund_quantity: u64,
}

#[derive(Serialize)]
pub struct ApplyRefundAmount {
    /// 【原订单金额】 原支付交易的订单总金额，单位为分，只能为整数。
    pub total: u64,
    pub refund: u64,
    /// #### 【退款出资账户及金额】
    /// 退款需要从指定账户出资时，传递此参数指定出资金额（币种的最小单位，只能为整数）。
    ///
    /// ##### 同时指定多个账户出资退款的使用场景需要满足以下条件：
    ///     1、未开通退款支出分离产品功能；
    ///     2、订单属于分账订单，且分账处于待分账或分账中状态。
    ///
    /// ##### 参数传递需要满足条件：
    ///     1、基本账户可用余额出资金额与基本账户不可用余额出资金额之和等于退款金额；
    ///     2、账户类型不能重复。
    // 上述任一条件不满足将返回错误
    pub from: Option<RefundFrom>,
    pub currency: WechatPayCurrency,
}

#[derive(Serialize)]
pub struct ApplyRefundCommand {
    #[cfg(feature = "platform")]
    pub sub_mchid: Option<String>,
    pub transaction_id: Option<String>,
    pub out_trade_no: Option<String>,
    pub out_refund_no: String,
    pub reason: Option<String>,
    ////【退款结果回调url】
    ///  异步接收微信支付退款结果通知的回调地址，通知url必须为外网可访问的url，不能携带参数。
    /// 如果参数中传了notify_url，则商户平台上配置的回调地址将不会生效，优先回调当前传的这个地址。
    pub notify_url: Option<String>,
    ///【退款资金来源】 若传递此参数则使用对应的资金账户退款，否则默认使用未结算资金退款（仅对老资金流商户适用）
    pub funds_account: Option<String>,

    pub amount: ApplyRefundAmount,
    pub goods_detail: Option<Vec<GoodsDetail>>,
}
