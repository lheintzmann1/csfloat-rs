pub mod listing;
pub mod item;
pub mod seller;
pub mod reference;
pub mod stickers;
pub mod buy_orders;
pub mod me;
pub mod stall;
pub mod auction;
pub mod statistics;
pub mod schema;

pub use listing::Listing;
pub use item::Item;
pub use seller::Seller;
pub use reference::Reference;
pub use stickers::{Sticker, StickerReference};
pub use buy_orders::BuyOrder;
pub use me::{Me, User, Preferences, Statistics as UserStatistics};
pub use stall::Stall;
pub use auction::{AuctionDetails, TopBid};
pub use statistics::Statistics;
pub use schema::{SchemaResponse, Weapon, Paint, Collection, WearCondition};

use serde::{Deserialize, Serialize};

/// Response from get_all_listings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListingsResponse {
    pub data: Vec<Listing>,
    pub cursor: Option<String>,
}

impl ListingsResponse {
    pub fn listings(self) -> Vec<Listing> {
        self.data
    }
}

/// Exchange rates response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRates {
    pub data: std::collections::HashMap<String, f64>,
}