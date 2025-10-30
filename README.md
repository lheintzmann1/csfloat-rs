# CSFloat Async API Client (Unofficial)

[![Apache-2.0 License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE-APACHE)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![Crates.io](https://img.shields.io/crates/v/csfloat-rs.svg)](https://crates.io/crates/csfloat-rs)
[![Documentation](https://docs.rs/csfloat-rs/badge.svg)](https://docs.rs/csfloat-rs)

An **unofficial**, **asynchronous** Rust library for interacting with the [CSFloat](https://csfloat.com) API. This library provides full support for programmatic access to listings, buy orders, user data, exchange rates, and more. All prices returned by the server are in **cents**.

## Key Features

* **Asynchronous design**: Built on `tokio` and `reqwest` for non-blocking I/O.
* **Type-safe**: Fully typed models using Rust's type system and `serde`.
* **Fetch listings**: Retrieve detailed listings with filters (price in cents, float, rarity, etc.).
* **Buy orders**: Get and manage buy orders for specific items.
* **User information**: Access your own profile, trades, and stall data.
* **Weapon pricing schema**: Get average prices for all weapon skins across different wear conditions.
* **Listing management**: Create, delete, and modify listings and buy orders.
* **Proxy support**: Optional SOCKS4/5 and HTTP(S) proxy support.
* **Error handling**: Clear error types with descriptive messages.
* **Builder pattern**: Ergonomic API for complex queries.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
csfloat-rs = "0.1"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

```rust
use csfloat_rs::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new("YOUR_API_KEY")?;
    
    // Fetch up to 50 listings priced between $1.00 and $10.00 (i.e., 100–1000 cents)
    let response = client
        .get_all_listings()
        .min_price(100)
        .max_price(1000)
        .limit(50)
        .send()
        .await?;
    
    for listing in response.listings() {
        println!(
            "ID: {}, Price: {} cents, Float: {:?}", 
            listing.id, 
            listing.price,
            listing.item.float_value
        );
    }
    
    Ok(())
}
```

## Examples

### Create a Buy Order

```rust
use csfloat_rs::{Client, client::CreateBuyOrderRequest};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new("YOUR_API_KEY")?;
    
    let request = CreateBuyOrderRequest {
        market_hash_name: "AK-47 | Redline (Field-Tested)".to_string(),
        max_price: 5000,  // 5000 cents = $50.00
        quantity: 1,
    };
    
    let buy_order = client.create_buy_order(request).await?;
    println!("Buy order created: {:?}", buy_order);
    
    Ok(())
}
```

### Get User Profile

```rust
use csfloat_rs::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new("YOUR_API_KEY")?;
    
    let me = client.get_me().await?;
    println!("Username: {:?}", me.user.username);
    println!("Balance: {:?} cents", me.user.balance);
    println!("Total trades: {:?}", me.user.statistics.map(|s| s.total_trades));
    
    Ok(())
}
```

### Advanced Listing Search

```rust
use csfloat_rs::{Client, client::{SortBy, Category}};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new("YOUR_API_KEY")?;
    
    let response = client
        .get_all_listings()
        .min_price(1000)
        .max_price(50000)
        .min_float(0.0)
        .max_float(0.15)
        .sort_by(SortBy::LowestFloat)
        .category(Category::StatTrak)
        .market_hash_name("AWP | Asiimov (Field-Tested)".to_string())
        .limit(20)
        .send()
        .await?;
    
    for listing in response.listings() {
        if let Some(float) = listing.item.float_value {
            println!("Listing {}: Float {:.8}", listing.id, float);
        }
    }
    
    Ok(())
}
```

### Using Proxy

```rust
use csfloat_rs::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::with_proxy(
        "YOUR_API_KEY",
        "socks5://user:pass@host:port"
    )?;
    
    let listings = client
        .get_all_listings()
        .limit(10)
        .send()
        .await?;
    
    println!("Found {} listings via proxy", listings.data.len());
    
    Ok(())
}
```

### Buy an Item

```rust
use csfloat_rs::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new("YOUR_API_KEY")?;
    
    let listing_id = 123456789;
    let total_price = 5000; // 5000 cents = $50.00
    
    let result = client.buy_now(total_price, listing_id).await?;
    println!("Purchase result: {:?}", result);
    
    Ok(())
}
```

### Get Weapon Pricing Schema

```rust
use csfloat_rs::{Client, models::WearCondition};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new("YOUR_API_KEY")?;
    
    // Get the complete schema with weapon pricing data
    let schema = client.get_schema().await?;
    
    println!("Total weapons: {}", schema.weapons.len());
    
    // Find AK-47 Redline pricing
    if let Some((_, ak47)) = schema.get_weapon_by_name("AK-47") {
        if let Some(redline) = ak47.get_paint_by_name("Redline") {
            println!("AK-47 Redline prices:");
            
            // Normal prices by wear condition
            for wear in [WearCondition::FactoryNew, WearCondition::MinimalWear, 
                        WearCondition::FieldTested, WearCondition::WellWorn, 
                        WearCondition::BattleScarred] {
                if let Some(price) = redline.price_for_wear(wear) {
                    if let Some(volume) = redline.volume_for_wear(wear) {
                        println!("  {} - ${:.2} (Volume: {})", 
                            wear.as_str(), price as f64 / 100.0, volume);
                    }
                }
            }
            
            // StatTrak prices if available
            if redline.has_stattrak() {
                println!("StatTrak prices:");
                for wear in [WearCondition::MinimalWear, WearCondition::FieldTested] {
                    if let Some(price) = redline.stattrak_price_for_wear(wear) {
                        println!("  StatTrak {} - ${:.2}", 
                            wear.as_str(), price as f64 / 100.0);
                    }
                }
            }
        }
    }
    
    // Find most expensive skins
    let mut expensive_skins: Vec<(&str, &str, u64)> = Vec::new();
    for (_, weapon) in &schema.weapons {
        for paint in weapon.paints.values() {
            if let Some(price) = paint.price_for_wear(WearCondition::FactoryNew) {
                if price > 100000 { // Over $1000
                    expensive_skins.push((&weapon.name, &paint.name, price));
                }
            }
        }
    }
    
    expensive_skins.sort_by(|a, b| b.2.cmp(&a.2));
    println!("Most expensive Factory New skins:");
    for (weapon, skin, price) in expensive_skins.iter().take(5) {
        println!("  {} | {} - ${:.2}", weapon, skin, *price as f64 / 100.0);
    }
    
    Ok(())
}
```

### Pagination

```rust
use csfloat_rs::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new("YOUR_API_KEY")?;
    
    let mut cursor: Option<String> = None;
    let mut all_listings = Vec::new();
    
    for _ in 0..5 {  // Fetch 5 pages
        let mut builder = client.get_all_listings().limit(50);
        
        if let Some(c) = cursor {
            builder = builder.cursor(c);
        }
        
        let response = builder.send().await?;
        all_listings.extend(response.data);
        
        cursor = response.cursor;
        if cursor.is_none() {
            break;  // No more pages
        }
    }
    
    println!("Fetched {} total listings", all_listings.len());
    
    Ok(())
}
```

## Core Methods

### Market Data

* `get_exchange_rates()` – Retrieve current exchange rates.
* `get_schema()` – Get weapon pricing schema with average prices (undocumented endpoint).
* `get_all_listings()` – List items with optional filters (returns builder).
* `get_specific_listing(listing_id)` – Get detailed info for a specific listing.
* `get_similar(listing_id)` – Get similar listings.
* `get_sales(market_hash_name, paint_index)` – Get sales history.

### Buy Orders

* `get_buy_orders(listing_id, limit)` – Retrieve buy orders for a listing.
* `get_my_buy_orders(page, limit)` – List your own buy orders.
* `create_buy_order(request)` – Place a buy order.
* `delete_buy_order(id)` – Cancel an existing buy order.

### User Data

* `get_me()` – Fetch authenticated user profile.
* `get_stall(user_id, limit)` – Get a user's stall (listed items).
* `get_inventory()` – Get your inventory.
* `get_watchlist(limit)` – Get your watchlist.
* `get_transactions(page, limit)` – Get transaction history.
* `get_trade_history(role, limit, page)` – Get trade history.

### Listing Management

* `create_listing(request)` – Create a new listing.
* `delete_listing(listing_id)` – Delete a listing.
* `update_listing_price(listing_id, price)` – Update listing price.
* `make_offer(listing_id, price)` – Make an offer on a listing.
* `buy_now(total_price, listing_id)` – Instantly buy a listing.

## Error Handling

The client uses a custom `Result` type and provides detailed error information:

```rust
use csfloat_rs::{Client, Error};

#[tokio::main]
async fn main() {
    let client = Client::new("YOUR_API_KEY").unwrap();
    
    match client.get_specific_listing(123456).await {
        Ok(listing) => println!("Found listing: {:?}", listing),
        Err(Error::NotFound) => println!("Listing not found"),
        Err(Error::Unauthorized) => println!("Invalid API key"),
        Err(Error::TooManyRequests) => println!("Rate limited, slow down!"),
        Err(e) => println!("Other error: {}", e),
    }
}
```

## Proxy Support

Support for HTTP, HTTPS, SOCKS4, and SOCKS5 proxies:

```rust
// HTTP proxy
let client = Client::with_proxy("API_KEY", "http://proxy:8080")?;

// SOCKS5 proxy with auth
let client = Client::with_proxy("API_KEY", "socks5://user:pass@proxy:1080")?;
```

## Contributing

Contributions are welcome! Feel free to submit a Pull Request.

## License

This project is licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [Apache 2.0](http://www.apache.org/licenses/LICENSE-2.0))
* MIT License ([LICENSE-MIT](LICENSE-MIT) or [MIT](http://opensource.org/licenses/MIT))

at your option.

## Disclaimer

This is an **unofficial** library and is not affiliated with or endorsed by CSFloat. Use at your own risk and ensure compliance with CSFloat's Terms of Service.
