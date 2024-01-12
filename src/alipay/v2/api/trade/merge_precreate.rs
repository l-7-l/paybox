use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PreOrderResult {
    pub app_id: String,
    pub out_trade_no: String,
    pub success: bool,
    pub result_code: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PrecreateOrderResponse {
    pub out_merge_no: Option<String>,
    pub pre_order_no: Option<String>,
    pub order_detail_results: Vec<PreOrderResult>,
}
