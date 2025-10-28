use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistics {
    pub total_sales: Option<i32>,
    pub total_purchases: Option<i32>,
    pub median_trade_time: Option<i32>,
    pub total_avoided_trades: Option<i32>,
    pub total_failed_trades: Option<i32>,
    pub total_verified_trades: Option<i32>,
    pub total_trades: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preferences {
    pub offers_enabled: Option<bool>,
    pub max_offer_discount: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirebaseMessaging {
    pub platform: Option<i32>,
    pub last_updated: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub steam_id: Option<String>,
    pub username: Option<String>,
    pub flags: Option<i32>,
    pub avatar: Option<String>,
    pub background_url: Option<String>,
    pub email: Option<String>,
    pub balance: Option<i64>,
    pub pending_balance: Option<i64>,
    pub stall_public: Option<bool>,
    pub away: Option<bool>,
    pub trade_token: Option<String>,
    pub payment_accounts: Option<serde_json::Value>,
    pub api_key: Option<String>,
    pub statistics: Option<Statistics>,
    pub preferences: Option<Preferences>,
    pub know_your_customer: Option<String>,
    pub extension_setup_at: Option<String>,
    pub firebase_messaging: Option<FirebaseMessaging>,
    pub stripe_connect: Option<serde_json::Value>,
    pub has_valid_steam_api_key: Option<bool>,
    pub obfuscated_id: Option<String>,
    pub online: Option<bool>,
    pub fee: Option<f64>,
    pub withdraw_fee: Option<f64>,
    pub subscriptions: Option<Vec<serde_json::Value>>,
    pub has_2fa: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Me {
    pub user: User,
    pub pending_offers: Option<i32>,
    pub actionable_trades: Option<i32>,
}
