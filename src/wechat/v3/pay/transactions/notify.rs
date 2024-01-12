use crate::wechat::pay::{trade_kind::TradeKind, trade_state::TradeState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyAmount {
    pub total: Option<u64>,
    pub payer_total: Option<u64>,
    pub currency: Option<String>,
    pub payer_currency: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecodedNotifyResource {
    #[cfg(feature = "platform")]
    pub sp_appid: String,
    #[cfg(feature = "platform")]
    pub sp_mchid: String,
    // #[cfg(feature = "platform")]
    // pub sub_appid: Option<String>,
    #[cfg(feature = "platform")]
    pub sub_mchid: String,
    pub out_trade_no: String,
    pub transaction_id: String,
    pub trade_type: Option<TradeKind>,
    pub trade_state: TradeState,
    pub bank_type: Option<String>,
    pub attach: Option<String>,
    pub success_time: Option<String>,
    pub amount: NotifyAmount,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubOrderAmount {
    pub total_amount: u64,
    pub currency: String,
    pub payer_amount: u64,
    pub payer_currency: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubOrder {
    pub mch_id: String,
    pub trade_type: TradeKind,
    pub trade_state: TradeState,
    pub bank_type: Option<String>,
    pub attach: String,
    pub success_time: String,
    pub transaction_id: String,
    pub out_trade_no: String,
    pub sub_mchid: String,
    pub sub_appid: String,
    pub amount: SubOrderAmount,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CombinedPayerInfo {
    pub openid: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DecodedCombinedNotifyResource {
    pub combine_appid: String,
    pub combine_mchid: String,
    pub combine_out_trade_no: String,
    pub sub_orders: Vec<SubOrder>,
}
