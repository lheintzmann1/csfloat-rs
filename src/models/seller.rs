use serde::{Deserialize, Serialize};
use super::Statistics;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seller {
    pub avatar: Option<String>,
    pub away: Option<bool>,
    pub flags: Option<i32>,
    pub has_valid_steam_api_key: Option<bool>,
    pub obfuscated_id: Option<String>,
    pub online: Option<bool>,
    pub stall_public: Option<bool>,
    pub statistics: Option<Statistics>,
    pub steam_id: Option<String>,
    pub username: Option<String>,
    pub verification_mode: Option<String>,
}
