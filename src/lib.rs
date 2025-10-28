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
//!     for listing in listings.listings {
//!         println!("ID: {}, Price: {} cents", listing.id, listing.price);
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