use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyOrder {
    pub id: Option<String>,
    pub created_at: Option<String>,
    pub expression: Option<String>,
    pub qty: Option<i32>,
    pub price: Option<i32>,
}
