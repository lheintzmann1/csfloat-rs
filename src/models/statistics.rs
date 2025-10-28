use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistics {
    pub median_trade_time: Option<f64>,
    pub total_avoided_trades: Option<i32>,
    pub total_failed_trades: Option<i32>,
    pub total_trades: Option<i32>,
    pub total_verified_trades: Option<i32>,
}
