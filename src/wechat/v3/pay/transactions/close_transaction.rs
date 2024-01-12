use serde::Serialize;

#[derive(Serialize)]
pub struct CloseSubOrderParams {
    pub mchid: String,
    pub out_trade_no: String,
    #[cfg(feature = "platform")]
    //  特约商户商户号
    pub sub_mchid: Option<String>,
    // 【服务商模式下，sub_mchid对应的sub_appid】
    #[cfg(feature = "platform")]
    pub sub_appid: Option<String>,
}

#[derive(Serialize)]
pub struct CloseCombinedOrderParams {
    pub combine_app_id: String,
    pub sub_orders: Option<Vec<CloseSubOrderParams>>,
}

#[derive(Serialize)]
pub struct CloseOrderParams {
    #[cfg(not(feature = "platform"))]
    mchid: String,

    #[cfg(feature = "platform")]
    sp_mchid: String,

    #[cfg(feature = "platform")]
    sub_mchid: String,
}

impl CloseOrderParams {
    #[cfg(feature = "platform")]
    pub fn new(mch_id: String, sub_mch_id: String) -> Self {
        Self {
            sp_mchid: mch_id,
            sub_mchid: sub_mch_id,
        }
    }

    #[cfg(not(feature = "platform"))]
    pub fn new(mch_id: String) -> Self {
        Self { mchid: mch_id }
    }
}
