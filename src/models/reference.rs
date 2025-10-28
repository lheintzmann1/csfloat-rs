use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    pub base_price: Option<f64>,
    pub float_factor: Option<f64>,
    pub predicted_price: Option<f64>,
    pub quantity: Option<i32>,
    pub last_updated: Option<String>,
}
