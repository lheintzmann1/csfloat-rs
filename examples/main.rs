use csfloat_rs::{
    Client, Result,
    client::{Category, CreateBuyOrderRequest, CreateListingRequest, ListingType, SortBy, TradeRole},
};

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();
    
    // Replace with your actual API key
    let api_key = std::env::var("CSFLOAT_API_KEY")
        .expect("CSFLOAT_API_KEY environment variable not set");
    
    let client = Client::new(api_key)?;
    
    // Example 1: Get exchange rates
    println!("\n=== Example 1: Exchange Rates ===");
    let rates = client.get_exchange_rates().await?;
    println!("Exchange rates: {:?}", rates);
    
    // Example 2: Get user profile
    println!("\n=== Example 2: User Profile ===");
    let me = client.get_me().await?;
    println!("Username: {:?}", me.user.username);
    println!("Balance: {:?} cents", me.user.balance);
    println!("Steam ID: {:?}", me.user.steam_id);
    
    // Example 3: Simple listing search
    println!("\n=== Example 3: Simple Listing Search ===");
    let response = client
        .get_all_listings()
        .min_price(100)
        .max_price(1000)
        .limit(5)
        .send()
        .await?;
    
    println!("Found {} listings:", response.data.len());
    for listing in &response.data {
        println!(
            "  - ID: {}, Price: {:.2} cents, Item: {:?}",
            listing.id,
            listing.price,
            listing.item.market_hash_name
        );
    }
    
    // Example 4: Advanced listing search with filters
    println!("\n=== Example 4: Advanced Search (StatTrak, Low Float) ===");
    let response = client
        .get_all_listings()
        .category(Category::StatTrak)
        .min_float(0.0)
        .max_float(0.10)
        .sort_by(SortBy::LowestFloat)
        .limit(3)
        .send()
        .await?;
    
    for listing in &response.data {
        if let Some(float) = listing.item.float_value {
            println!(
                "  - {:?} - Float: {:.8}",
                listing.item.market_hash_name,
                float
            );
        }
    }
    
    // Example 5: Get specific listing
    println!("\n=== Example 5: Get Specific Listing ===");
    if let Some(first_listing) = response.data.first() {
        let detailed = client.get_specific_listing(&first_listing.id).await?;
        println!("Detailed listing: {}", detailed.id);
        println!("  Seller: {:?}", detailed.seller.username);
        println!("  State: {}", detailed.state);
        if let Some(watchers) = detailed.watchers {
            println!("  Watchers: {}", watchers);
        }
    }
    
    // Example 6: Get similar listings
    println!("\n=== Example 6: Similar Listings ===");
    if let Some(first_listing) = response.data.first() {
        match client.get_similar(&first_listing.id).await {
            Ok(similar) => {
                println!("Found {} similar listings", similar.len());
                for listing in similar.iter().take(3) {
                    println!("  - ID: {}, Price: {:.2}", listing.id, listing.price);
                }
            }
            Err(e) => println!("Could not fetch similar listings: {}", e),
        }
    }
    
    // Example 7: Search by market hash name
    println!("\n=== Example 7: Search by Market Hash Name ===");
    let response = client
        .get_all_listings()
        .market_hash_name("AK-47 | Redline (Field-Tested)".to_string())
        .limit(5)
        .sort_by(SortBy::LowestPrice)
        .send()
        .await?;
    
    println!("AK-47 Redline listings:");
    for listing in &response.data {
        println!("  - Price: {:.2} cents", listing.price);
    }
    
    // Example 8: Buy orders
    println!("\n=== Example 8: Buy Orders ===");
    if let Some(listing) = response.data.first() {
        match client.get_buy_orders(&listing.id, 5).await {
            Ok(orders) => {
                println!("Buy orders for listing {}:", listing.id);
                for order in orders {
                    println!("  - Price: {:?}, Qty: {:?}", order.price, order.qty);
                }
            }
            Err(e) => println!("Could not fetch buy orders: {}", e),
        }
    }
    
    // Example 9: Get user's buy orders
    println!("\n=== Example 9: My Buy Orders ===");
    match client.get_my_buy_orders(0, 5).await {
        Ok(my_orders) => {
            println!("Your buy orders: {:?}", my_orders);
        }
        Err(e) => println!("Could not fetch your buy orders: {}", e),
    }
    
    // Example 10: Get stall (user's listings)
    println!("\n=== Example 10: User Stall ===");
    if let Some(steam_id) = &me.user.steam_id {
        match client.get_stall(steam_id, 10).await {
            Ok(stall) => {
                println!("Your stall has {} listings", stall.data.len());
                for listing in stall.listings().iter().take(3) {
                    println!("  - {:?} - {:.2} cents", listing.item.market_hash_name, listing.price);
                }
            }
            Err(e) => println!("Could not fetch stall: {}", e),
        }
    }
    
    // Example 11: Get inventory
    println!("\n=== Example 11: Inventory ===");
    match client.get_inventory().await {
        Ok(inventory) => {
            println!("Inventory data retrieved (raw): {:?}", inventory);
        }
        Err(e) => println!("Could not fetch inventory: {}", e),
    }
    
    // Example 12: Get watchlist
    println!("\n=== Example 12: Watchlist ===");
    match client.get_watchlist(10).await {
        Ok(watchlist) => {
            println!("Watchlist: {:?}", watchlist);
        }
        Err(e) => println!("Could not fetch watchlist: {}", e),
    }
    
    // Example 13: Pagination
    println!("\n=== Example 13: Pagination ===");
    let mut cursor: Option<String> = None;
    let mut total_items = 0;
    
    for page in 0..3 {
        let mut builder = client.get_all_listings().limit(10);
        
        if let Some(c) = cursor {
            builder = builder.cursor(c);
        }
        
        let response = builder.send().await?;
        total_items += response.data.len();
        println!("Page {}: {} items", page + 1, response.data.len());
        
        cursor = response.cursor;
        if cursor.is_none() {
            println!("No more pages available");
            break;
        }
    }
    println!("Total items fetched: {}", total_items);
    
    // Example 14: Get transactions
    println!("\n=== Example 14: Transactions ===");
    match client.get_transactions(0, 5).await {
        Ok(transactions) => {
            println!("Recent transactions: {:?}", transactions);
        }
        Err(e) => println!("Could not fetch transactions: {}", e),
    }
    
    // Example 15: Get trade history
    println!("\n=== Example 15: Trade History ===");
    match client.get_trade_history(TradeRole::Buyer, 5, 0).await {
        Ok(history) => {
            println!("Trade history (as buyer): {:?}", history);
        }
        Err(e) => println!("Could not fetch trade history: {}", e),
    }
    
    // Example 16: Get sales history for an item
    println!("\n=== Example 16: Sales History ===");
    match client.get_sales("AK-47 | Redline (Field-Tested)", None).await {
        Ok(sales) => {
            println!("Sales history: {:?}", sales);
        }
        Err(e) => println!("Could not fetch sales history: {}", e),
    }
    
    // Example 17: Create buy order (commented out - requires action)
    println!("\n=== Example 17: Create Buy Order (Example) ===");
    println!("To create a buy order, uncomment the code below:");
    /*
    let buy_order_request = CreateBuyOrderRequest {
        market_hash_name: "AK-47 | Redline (Field-Tested)".to_string(),
        max_price: 5000,  // 50.00 USD in cents
        quantity: 1,
    };
    
    match client.create_buy_order(buy_order_request).await {
        Ok(result) => println!("Buy order created: {:?}", result),
        Err(e) => println!("Failed to create buy order: {}", e),
    }
    */
    
    // Example 18: Create listing (commented out - requires action)
    println!("\n=== Example 18: Create Listing (Example) ===");
    println!("To create a listing, uncomment the code below:");
    /*
    let listing_request = CreateListingRequest {
        asset_id: "YOUR_ASSET_ID".to_string(),
        price: 10000.0,  // 100.00 USD in cents
        listing_type: "buy_now".to_string(),
        max_offer_discount: Some(10),  // 10% max discount
        reserve_price: None,
        duration_days: None,
        description: "Great item!".to_string(),
        private: false,
    };
    
    match client.create_listing(listing_request).await {
        Ok(result) => println!("Listing created: {:?}", result),
        Err(e) => println!("Failed to create listing: {}", e),
    }
    */
    
    // Example 19: Make an offer (commented out - requires action)
    println!("\n=== Example 19: Make Offer (Example) ===");
    println!("To make an offer, uncomment the code below:");
    /*
    let listing_id = 123456789;
    let offer_price = 9500; // 95.00 USD in cents
    
    match client.make_offer(listing_id, offer_price).await {
        Ok(result) => println!("Offer made: {:?}", result),
        Err(e) => println!("Failed to make offer: {}", e),
    }
    */
    
    // Example 20: Buy now (commented out - requires action)
    println!("\n=== Example 20: Buy Now (Example) ===");
    println!("To buy an item, uncomment the code below:");
    /*
    let listing_id = 123456789;
    let total_price = 10000; // 100.00 USD in cents
    
    match client.buy_now(total_price, listing_id).await {
        Ok(result) => println!("Purchase successful: {:?}", result),
        Err(e) => println!("Failed to buy: {}", e),
    }
    */
    
    // Example 21: Update listing price (commented out - requires action)
    println!("\n=== Example 21: Update Listing Price (Example) ===");
    println!("To update a listing price, uncomment the code below:");
    /*
    let listing_id = 123456789;
    let new_price = 9500; // 95.00 USD in cents
    
    match client.update_listing_price(listing_id, new_price).await {
        Ok(result) => println!("Price updated: {:?}", result),
        Err(e) => println!("Failed to update price: {}", e),
    }
    */
    
    // Example 22: Delete listing (commented out - requires action)
    println!("\n=== Example 22: Delete Listing (Example) ===");
    println!("To delete a listing, uncomment the code below:");
    /*
    let listing_id = 123456789;
    
    match client.delete_listing(listing_id).await {
        Ok(result) => println!("Listing deleted: {:?}", result),
        Err(e) => println!("Failed to delete listing: {}", e),
    }
    */
    
    // Example 23: Delete buy order (commented out - requires action)
    println!("\n=== Example 23: Delete Buy Order (Example) ===");
    println!("To delete a buy order, uncomment the code below:");
    /*
    let buy_order_id = 123456789;
    
    match client.delete_buy_order(buy_order_id).await {
        Ok(result) => println!("Buy order deleted: {:?}", result),
        Err(e) => println!("Failed to delete buy order: {}", e),
    }
    */
    
    // Example 24: Complex search with multiple filters
    println!("\n=== Example 24: Complex Search ===");
    let response = client
        .get_all_listings()
        .market_hash_name("AWP | Asiimov (Field-Tested)".to_string())
        .min_float(0.18)
        .max_float(0.25)
        .min_price(5000)
        .max_price(15000)
        .sort_by(SortBy::BestDeal)
        .category(Category::Normal)
        .limit(10)
        .send()
        .await?;
    
    println!("Complex search results:");
    for listing in &response.data {
        if let Some(float) = listing.item.float_value {
            println!(
                "  - Price: {:.2}, Float: {:.8}, Seller: {:?}",
                listing.price,
                float,
                listing.seller.username
            );
        }
    }
    
    // Example 25: Error handling
    println!("\n=== Example 25: Error Handling ===");
    match client.get_specific_listing("999999999999").await {
        Ok(listing) => println!("Found listing: {}", listing.id),
        Err(csfloat_rs::Error::NotFound) => println!("Listing not found (expected)"),
        Err(csfloat_rs::Error::Unauthorized) => println!("Invalid API key"),
        Err(csfloat_rs::Error::TooManyRequests) => println!("Rate limited"),
        Err(e) => println!("Other error: {}", e),
    }
    
    println!("\n=== All Examples Completed ===");
    Ok(())
}
