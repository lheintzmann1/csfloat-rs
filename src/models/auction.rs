use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopBid {
    pub id: Option<String>,
    pub created_at: Option<String>,
    pub price: Option<i32>,
    pub contract_id: Option<String>,
    pub state: Option<String>,
    pub active: Option<bool>,
    pub obfuscated_buyer_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionDetails {
    pub reserve_price: Option<f64>,
    pub top_bid: Option<TopBid>,
    pub expires_at: Option<String>,
    pub min_next_bid: Option<f64>,
}
