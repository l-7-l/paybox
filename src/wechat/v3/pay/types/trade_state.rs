use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TradeState {
    Success,
    Refund,
    NotPay,
    Closed,
    Revoked,
    UserPaying,
    PayError,
}

impl TradeState {
    /// Returns `true` if the trade state is [`Success`].
    ///
    /// [`Success`]: TradeState::Success
    #[must_use]
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success)
    }
}
