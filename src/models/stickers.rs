use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickerReference {
    pub price: Option<f64>,
    pub quantity: Option<i32>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sticker {
    #[serde(rename = "stickerId")]
    pub sticker_id: Option<i32>,
    pub slot: Option<i32>,
    pub wear: Option<f64>,
    pub offset_x: Option<f64>,
    pub offset_y: Option<f64>,
    pub icon_url: Option<String>,
    pub name: Option<String>,
    pub reference: Option<StickerReference>,
}
