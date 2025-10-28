use serde::{Deserialize, Serialize};
use super::Sticker;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub asset_id: Option<String>,
    pub def_index: Option<i32>,
    pub paint_index: Option<i32>,
    pub paint_seed: Option<i32>,
    pub float_value: Option<f64>,
    pub icon_url: Option<String>,
    pub d_param: Option<String>,
    pub is_stattrak: Option<bool>,
    pub is_souvenir: Option<bool>,
    pub rarity: Option<i32>,
    pub quality: Option<i32>,
    pub market_hash_name: Option<String>,
    pub low_rank: Option<i32>,
    pub high_rank: Option<i32>,
    pub stickers: Option<Vec<Sticker>>,
    pub tradable: Option<i32>,
    pub inspect_link: Option<String>,
    pub has_screenshot: Option<bool>,
    pub cs2_screenshot_id: Option<String>,
    pub cs2_screenshot_at: Option<String>,
    pub is_commodity: Option<bool>,
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    pub rarity_name: Option<String>,
    pub type_name: Option<String>,
    pub item_name: Option<String>,
    pub wear_name: Option<String>,
    pub description: Option<String>,
    pub collection: Option<String>,
    pub badges: Option<Vec<serde_json::Value>>,
    pub serialized_inspect: Option<String>,
    pub gs_sig: Option<String>,
}
