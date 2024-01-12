use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct OrderDetail {
    /// 订单明细的应用唯一标识（16位纯数字），指商家的app_id。
    pub app_id: String,
    /// 商户订单号,64个字符以内、只能包含字母、数字、下划线；需保证在商户端不重复
    pub out_trade_no: String,
    /// 卖家支付宝用户ID。
    /// 如果该值与seller_logon_id同时为空，则卖家默认为app_id对应的支付宝用户ID
    pub seller_id: Option<String>,
    ///  卖家支付宝logon_id。 支持手机和Email格式,如果该值与seller_id同时传入,将以seller_id为准
    pub seller_logon_id: Option<String>,

    /// 销售产品码，商家和支付宝签约的产品码，APP支付功能中该值固定为： QUICK_MSECURITY_PAY
    pub product_code: String,
    /// 订单总金额，单位为元，精确到小数点后两位，取值范围[0.01,100000000]
    pub total_amount: Decimal,
    /// 订单标题
    pub subject: String,
    /// 对交易或商品的描述
    /// Iphone6 16G
    pub body: Option<String>,

    ///商品的展示地址
    /// http://www.alipay.com/xxx.jpg
    pub show_url: Option<String>,
}
