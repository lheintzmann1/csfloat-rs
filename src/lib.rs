//! # CSFloat Async API Client (Unofficial)
//!
//! An **unofficial**, **asynchronous** Rust library for interacting with the [CSFloat](https://csfloat.com) API.
//! This library provides full support for programmatic access to listings, buy orders, user data, exchange rates, and more.
//! All prices returned by the server are in **cents**.
//!
//! ## Key Features
//!
//! * **Asynchronous design**: Built on `tokio` and `reqwest` for non-blocking I/O.
//! * **Fetch listings**: Retrieve detailed listings with filters (price in cents, float, rarity, etc.).
//! * **Buy orders**: Get and manage buy orders for specific items.
//! * **User information**: Access your own profile, trades, and stall data.
//! * **Weapon pricing schema**: Get average prices for all weapon skins across different wear conditions.
//! * **Listing management**: Create, delete, and modify listings and buy orders.
//! * **Proxy support**: Optional SOCKS4/5 and HTTP(S) proxy support.
//! * **Error handling**: Clear exceptions with descriptive messages.
//!
//! ## Quick Start
//!
//! ```no_run
//! use csfloat_rs::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new("YOUR_API_KEY")?;
//!     
//!     // Fetch up to 50 listings priced between $1.00 and $10.00 (i.e., 100–1000 cents)
//!     let listings = client.get_all_listings()
//!         .min_price(100)
//!         .max_price(1000)
//!         .send()
//!         .await?;
//!     
//!     for listing in listings.listings() {
//!         println!("ID: {}, Price: {} cents", listing.id, listing.price);
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Schema and Pricing Data
//!
//! Access weapon pricing data with the undocumented schema endpoint:
//!
//! ```no_run
//! use csfloat_rs::{Client, models::WearCondition};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new("YOUR_API_KEY")?;
//!     
//!     // Get schema with weapon pricing data
//!     let schema = client.get_schema().await?;
//!     
//!     // Find AK-47 Redline pricing
//!     if let Some((_, ak47)) = schema.get_weapon_by_name("AK-47") {
//!         if let Some(redline) = ak47.get_paint_by_name("Redline") {
//!             if let Some(price) = redline.price_for_wear(WearCondition::MinimalWear) {
//!                 println!("AK-47 Redline MW: ${:.2}", price as f64 / 100.0);
//!             }
//!         }
//!     }
//!     
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod models;

pub use client::Client;
pub use error::{Error, Result};