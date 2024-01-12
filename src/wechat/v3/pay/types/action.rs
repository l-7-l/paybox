use super::WechatPayMethod;

#[allow(unused)]
#[derive(Debug)]
pub enum WechatPayAction {
    NewTransaction(WechatPayMethod),
    /// 收款码
    MicroPay,
    /// 关闭订单
    CloseTransactionById(String),
    // 微信支付单号查询订单
    FindTransactionById(String),
    // 通过商户订单号查询订单
    FindTransactionByCustomId(String),
    /// 查询退款订单
    FindRefundByCustomId(String),
    /// 退款
    Refund,
    /// 撤销订单 - V3
    ReverseOrder,
    /// 转换短链接
    ShortUrl,
}

#[allow(unused)]
impl WechatPayAction {
    pub fn api_path(&self) -> String {
        use WechatPayAction::*;
        match self {
            NewTransaction(wechat_pay_method) => {
                let path = if cfg!(feature = "platform") {
                    "/v3/pay/partner/transactions"
                } else {
                    "/v3/pay/transactions"
                };

                format!("{path}{}", wechat_pay_method.channel_name())
            }

            CloseTransactionById(id) => {
                format!("/v3/pay/transactions/out-trade-no/{id}/close")
            }

            FindTransactionByCustomId(custom_id) => {
                format!("/v3/pay/transactions/out-trade-no/{custom_id}")
            }

            FindTransactionById(id) => format!("/v3/pay/transactions/id/{id}"),

            Refund => "/v3/refund/domestic/refunds".to_owned(),
            FindRefundByCustomId(custom_id) => {
                format!("/v3/refund/domestic/refunds/{custom_id}")
            }

            MicroPay => "/pay/micropay".to_owned(),
            ShortUrl => "/tools/shorturl".to_owned(),
            ReverseOrder => "/secapi/pay/reverse".to_owned(),
        }
    }
}
