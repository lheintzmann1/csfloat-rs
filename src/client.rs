use crate::error::{Error, Result};
use crate::models::*;
use reqwest::{Client as HttpClient, Method, Proxy, RequestBuilder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const API_URL: &str = "https://csfloat.com/api/v1";

/// CSFloat API Client
pub struct Client {
    http_client: HttpClient,
    api_key: String,
}

impl Client {
    /// Create a new CSFloat client with API key
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        let http_client = HttpClient::builder()
            .build()
            .map_err(Error::RequestError)?;

        Ok(Self {
            http_client,
            api_key: api_key.into(),
        })
    }

    /// Create a new CSFloat client with API key and proxy
    pub fn with_proxy(api_key: impl Into<String>, proxy_url: impl AsRef<str>) -> Result<Self> {
        let proxy = Proxy::all(proxy_url.as_ref())
            .map_err(|_| Error::InvalidProxy(proxy_url.as_ref().to_string()))?;

        let http_client = HttpClient::builder()
            .proxy(proxy)
            .build()
            .map_err(Error::RequestError)?;

        Ok(Self {
            http_client,
            api_key: api_key.into(),
        })
    }

    /// Internal request method
    async fn request<T: for<'de> Deserialize<'de>>(
        &self,
        method: Method,
        path: &str,
        json_body: Option<&impl Serialize>,
    ) -> Result<T> {
        let url = format!("{}{}", API_URL, path);
        
        let mut request = self
            .http_client
            .request(method, &url)
            .header("Authorization", &self.api_key);

        if let Some(body) = json_body {
            request = request.json(body);
        }

        let response = request.send().await?;
        let status = response.status();

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(Error::from_status(status.as_u16(), body));
        }

        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if !content_type.contains("application/json") {
            return Err(Error::UnexpectedContentType(content_type.to_string()));
        }

        let data = response.json().await?;
        Ok(data)
    }

    /// Get exchange rates
    pub async fn get_exchange_rates(&self) -> Result<ExchangeRates> {
        self.request(Method::GET, "/meta/exchange-rates", None::<&()>)
            .await
    }

    /// Get authenticated user profile
    pub async fn get_me(&self) -> Result<Me> {
        self.request(Method::GET, "/me", None::<&()>).await
    }

    /// Get user's location
    pub async fn get_location(&self) -> Result<serde_json::Value> {
        self.request(Method::GET, "/meta/location", None::<&()>)
            .await
    }

    /// Get transactions
    pub async fn get_transactions(&self, page: u32, limit: u32) -> Result<serde_json::Value> {
        let path = format!("/me/transactions?page={}&limit={}&order=desc", page, limit);
        self.request(Method::GET, &path, None::<&()>).await
    }

    /// Get account standing
    pub async fn get_account_standing(&self) -> Result<serde_json::Value> {
        self.request(Method::GET, "/me/account-standing", None::<&()>)
            .await
    }

    /// Get pending trades
    pub async fn get_pending_trades(&self, limit: u32, page: u32) -> Result<serde_json::Value> {
        let path = format!("/me/trades?state=pending&limit={}&page={}", limit, page);
        self.request(Method::GET, &path, None::<&()>).await
    }

    /// Get similar listings
    pub async fn get_similar(&self, listing_id: &str) -> Result<Vec<Listing>> {
        let path = format!("/listings/{}/similar", listing_id);
        self.request(Method::GET, &path, None::<&()>).await
    }

    /// Get buy orders for a listing
    pub async fn get_buy_orders(&self, listing_id: &str, limit: u32) -> Result<Vec<BuyOrder>> {
        let path = format!("/listings/{}/buy-orders?limit={}", listing_id, limit);
        self.request(Method::GET, &path, None::<&()>).await
    }

    /// Get user's own buy orders
    pub async fn get_my_buy_orders(&self, page: u32, limit: u32) -> Result<serde_json::Value> {
        let path = format!("/me/buy-orders?page={}&limit={}&order=desc", page, limit);
        self.request(Method::GET, &path, None::<&()>).await
    }

    /// Get sales history
    pub async fn get_sales(
        &self,
        market_hash_name: &str,
        paint_index: Option<i32>,
    ) -> Result<serde_json::Value> {
        let mut path = format!("/history/{}/sales", market_hash_name);
        if let Some(idx) = paint_index {
            path.push_str(&format!("?paint_index={}", idx));
        }
        self.request(Method::GET, &path, None::<&()>).await
    }

    /// Get all listings with builder pattern
    pub fn get_all_listings(&self) -> ListingsRequestBuilder {
        ListingsRequestBuilder::new(self)
    }

    /// Get specific listing
    pub async fn get_specific_listing(&self, listing_id: &str) -> Result<Listing> {
        let path = format!("/listings/{}", listing_id);
        self.request(Method::GET, &path, None::<&()>).await
    }

    /// Get user's stall
    pub async fn get_stall(&self, user_id: &str, limit: u32) -> Result<Stall> {
        let path = format!("/users/{}/stall?limit={}", user_id, limit);
        self.request(Method::GET, &path, None::<&()>).await
    }

    /// Get inventory
    pub async fn get_inventory(&self) -> Result<serde_json::Value> {
        self.request(Method::GET, "/me/inventory", None::<&()>)
            .await
    }

    /// Get watchlist
    pub async fn get_watchlist(&self, limit: u32) -> Result<serde_json::Value> {
        let path = format!("/me/watchlist?limit={}", limit);
        self.request(Method::GET, &path, None::<&()>).await
    }

    /// Get offers
    pub async fn get_offers(&self, limit: u32) -> Result<serde_json::Value> {
        let path = format!("/me/offers-timeline?limit={}", limit);
        self.request(Method::GET, &path, None::<&()>).await
    }

    /// Get trade history
    pub async fn get_trade_history(
        &self,
        role: TradeRole,
        limit: u32,
        page: u32,
    ) -> Result<serde_json::Value> {
        let role_str = role.as_str();
        let path = format!(
            "/me/trades?role={}&state=failed,cancelled,verified&limit={}&page={}",
            role_str, limit, page
        );
        self.request(Method::GET, &path, None::<&()>).await
    }

    /// Delete a listing
    pub async fn delete_listing(&self, listing_id: &str) -> Result<serde_json::Value> {
        let path = format!("/listings/{}", listing_id);
        self.request(Method::DELETE, &path, None::<&()>).await
    }

    /// Delete a buy order
    pub async fn delete_buy_order(&self, id: &str) -> Result<serde_json::Value> {
        let path = format!("/buy-orders/{}", id);
        self.request(Method::DELETE, &path, None::<&()>).await
    }

    /// Delete from watchlist
    pub async fn delete_watchlist(&self, id: i64) -> Result<serde_json::Value> {
        let path = format!("/listings/{}/watchlist", id);
        self.request(Method::DELETE, &path, None::<&()>).await
    }

    /// Create a listing
    pub async fn create_listing(&self, request: CreateListingRequest) -> Result<serde_json::Value> {
        self.request(Method::POST, "/listings", Some(&request))
            .await
    }

    /// Create a buy order
    pub async fn create_buy_order(&self, request: CreateBuyOrderRequest) -> Result<serde_json::Value> {
        self.request(Method::POST, "/buy-orders", Some(&request))
            .await
    }

    /// Make an offer
    pub async fn make_offer(&self, listing_id: &str, price: i32) -> Result<serde_json::Value> {
        let request = MakeOfferRequest {
            contract_id: listing_id.to_string(),
            price,
            cancel_previous_offer: false,
        };
        self.request(Method::POST, "/offers", Some(&request))
            .await
    }

    /// Buy now
    pub async fn buy_now(&self, total_price: i32, listing_id: &str) -> Result<serde_json::Value> {
        let request = BuyNowRequest {
            total_price,
            contract_ids: vec![listing_id.to_string()],
        };
        self.request(Method::POST, "/listings/buy", Some(&request))
            .await
    }

    /// Accept sales
    pub async fn accept_sale(&self, trade_ids: Vec<String>) -> Result<serde_json::Value> {
        let request = AcceptSaleRequest { trade_ids };
        self.request(Method::POST, "/trades/bulk/accept", Some(&request))
            .await
    }

    /// Update listing price
    pub async fn update_listing_price(&self, listing_id: &str, price: i32) -> Result<serde_json::Value> {
        let request = UpdatePriceRequest { price };
        let path = format!("/listings/{}", listing_id);
        self.request(Method::PATCH, &path, Some(&request))
            .await
    }
}

/// Builder for listings requests
pub struct ListingsRequestBuilder<'a> {
    client: &'a Client,
    min_price: Option<i32>,
    max_price: Option<i32>,
    cursor: Option<String>,
    limit: u32,
    sort_by: SortBy,
    category: Category,
    def_index: Option<Vec<i32>>,
    min_float: Option<f64>,
    max_float: Option<f64>,
    rarity: Option<String>,
    paint_seed: Option<i32>,
    paint_index: Option<i32>,
    user_id: Option<String>,
    collection: Option<String>,
    market_hash_name: Option<String>,
    listing_type: ListingType,
}

impl<'a> ListingsRequestBuilder<'a> {
    fn new(client: &'a Client) -> Self {
        Self {
            client,
            min_price: None,
            max_price: None,
            cursor: None,
            limit: 50,
            sort_by: SortBy::BestDeal,
            category: Category::Any,
            def_index: None,
            min_float: None,
            max_float: None,
            rarity: None,
            paint_seed: None,
            paint_index: None,
            user_id: None,
            collection: None,
            market_hash_name: None,
            listing_type: ListingType::BuyNow,
        }
    }

    pub fn min_price(mut self, price: i32) -> Self {
        self.min_price = Some(price);
        self
    }

    pub fn max_price(mut self, price: i32) -> Self {
        self.max_price = Some(price);
        self
    }

    pub fn cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }

    pub fn sort_by(mut self, sort: SortBy) -> Self {
        self.sort_by = sort;
        self
    }

    pub fn category(mut self, cat: Category) -> Self {
        self.category = cat;
        self
    }

    pub fn def_index(mut self, indices: Vec<i32>) -> Self {
        self.def_index = Some(indices);
        self
    }

    pub fn min_float(mut self, float: f64) -> Self {
        self.min_float = Some(float);
        self
    }

    pub fn max_float(mut self, float: f64) -> Self {
        self.max_float = Some(float);
        self
    }

    pub fn rarity(mut self, rarity: String) -> Self {
        self.rarity = Some(rarity);
        self
    }

    pub fn paint_seed(mut self, seed: i32) -> Self {
        self.paint_seed = Some(seed);
        self
    }

    pub fn paint_index(mut self, index: i32) -> Self {
        self.paint_index = Some(index);
        self
    }

    pub fn user_id(mut self, id: String) -> Self {
        self.user_id = Some(id);
        self
    }

    pub fn collection(mut self, collection: String) -> Self {
        self.collection = Some(collection);
        self
    }

    pub fn market_hash_name(mut self, name: String) -> Self {
        self.market_hash_name = Some(name);
        self
    }

    pub fn listing_type(mut self, t: ListingType) -> Self {
        self.listing_type = t;
        self
    }

    pub async fn send(self) -> Result<ListingsResponse> {
        let mut path = format!(
            "/listings?limit={}&sort_by={}&category={}&type={}",
            self.limit,
            self.sort_by.as_str(),
            self.category.as_u8(),
            self.listing_type.as_str()
        );

        if let Some(cursor) = &self.cursor {
            path.push_str(&format!("&cursor={}", cursor));
        }
        if let Some(min) = self.min_price {
            path.push_str(&format!("&min_price={}", min));
        }
        if let Some(max) = self.max_price {
            path.push_str(&format!("&max_price={}", max));
        }
        if let Some(indices) = &self.def_index {
            let indices_str = indices
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(",");
            path.push_str(&format!("&def_index={}", indices_str));
        }
        if let Some(min) = self.min_float {
            path.push_str(&format!("&min_float={}", min));
        }
        if let Some(max) = self.max_float {
            path.push_str(&format!("&max_float={}", max));
        }
        if let Some(r) = &self.rarity {
            path.push_str(&format!("&rarity={}", r));
        }
        if let Some(seed) = self.paint_seed {
            path.push_str(&format!("&paint_seed={}", seed));
        }
        if let Some(idx) = self.paint_index {
            path.push_str(&format!("&paint_index={}", idx));
        }
        if let Some(id) = &self.user_id {
            path.push_str(&format!("&user_id={}", id));
        }
        if let Some(col) = &self.collection {
            path.push_str(&format!("&collection={}", col));
        }
        if let Some(name) = &self.market_hash_name {
            path.push_str(&format!("&market_hash_name={}", name));
        }

        self.client
            .request(Method::GET, &path, None::<&()>)
            .await
    }
}

// Enums and request types

#[derive(Debug, Clone, Copy)]
pub enum SortBy {
    LowestPrice,
    HighestPrice,
    MostRecent,
    ExpiresSoon,
    LowestFloat,
    HighestFloat,
    BestDeal,
    HighestDiscount,
    FloatRank,
    NumBids,
}

impl SortBy {
    fn as_str(&self) -> &str {
        match self {
            Self::LowestPrice => "lowest_price",
            Self::HighestPrice => "highest_price",
            Self::MostRecent => "most_recent",
            Self::ExpiresSoon => "expires_soon",
            Self::LowestFloat => "lowest_float",
            Self::HighestFloat => "highest_float",
            Self::BestDeal => "best_deal",
            Self::HighestDiscount => "highest_discount",
            Self::FloatRank => "float_rank",
            Self::NumBids => "num_bids",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Category {
    Any = 0,
    Normal = 1,
    StatTrak = 2,
    Souvenir = 3,
}

impl Category {
    fn as_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ListingType {
    BuyNow,
    Auction,
}

impl ListingType {
    fn as_str(&self) -> &str {
        match self {
            Self::BuyNow => "buy_now",
            Self::Auction => "auction",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TradeRole {
    Seller,
    Buyer,
}

impl TradeRole {
    fn as_str(&self) -> &str {
        match self {
            Self::Seller => "seller",
            Self::Buyer => "buyer",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateListingRequest {
    pub asset_id: String,
    pub price: f64,
    #[serde(rename = "type")]
    pub listing_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_offer_discount: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserve_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_days: Option<i32>,
    pub description: String,
    pub private: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateBuyOrderRequest {
    pub market_hash_name: String,
    pub max_price: i32,
    pub quantity: i32,
}

#[derive(Debug, Clone, Serialize)]
struct MakeOfferRequest {
    contract_id: String,
    price: i32,
    cancel_previous_offer: bool,
}

#[derive(Debug, Clone, Serialize)]
struct BuyNowRequest {
    total_price: i32,
    contract_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct AcceptSaleRequest {
    trade_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct UpdatePriceRequest {
    price: i32,
}
