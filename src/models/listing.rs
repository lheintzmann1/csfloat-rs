use serde::{Deserialize, Serialize};
use super::{Seller, Reference, Item, AuctionDetails};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    pub id: String,
    pub created_at: String,
    #[serde(rename = "type")]
    pub listing_type: String,
    pub price: f64,
    pub description: Option<String>,
    pub state: String,
    pub seller: Seller,
    pub reference: Option<Reference>,
    pub item: Item,
    pub is_seller: Option<bool>,
    pub min_offer_price: Option<f64>,
    pub max_offer_discount: Option<f64>,
    pub is_watchlisted: Option<bool>,
    pub watchers: Option<i32>,
    pub auction_details: Option<AuctionDetails>,
    pub sold_at: Option<String>,
}
