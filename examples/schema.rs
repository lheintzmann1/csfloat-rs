use csfloat_rs::{Client, Result};
use csfloat_rs::models::WearCondition;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();
    
    // Replace with your actual API key
    let api_key = std::env::var("CSFLOAT_API_KEY")
        .expect("CSFLOAT_API_KEY environment variable not set");
    
    let client = Client::new(api_key)?;
    
    println!("=== CSFloat Schema Example ===");
    println!("Testing the undocumented /schema endpoint...\n");
    
    // Get the schema data
    let schema = client.get_schema().await?;
    
    println!("🎯 Schema loaded successfully!");
    println!("📊 Total weapons: {}", schema.weapons.len());
    println!("📦 Total collections: {}", schema.collections.len());
    
    // Example 1: Show some popular weapons
    println!("\n=== Popular Weapons ===");
    let popular_weapons = ["AK-47", "M4A4", "AWP", "Desert Eagle", "Glock-18"];
    
    for weapon_name in &popular_weapons {
        if let Some((id, weapon)) = schema.get_weapon_by_name(weapon_name) {
            println!("🔫 {} (ID: {}) - {} paints available", 
                weapon.name, id, weapon.paints.len());
        }
    }
    
    // Example 2: Show AK-47 Redline pricing
    println!("\n=== AK-47 Redline Pricing ===");
    if let Some((_, ak47)) = schema.get_weapon_by_name("AK-47") {
        if let Some(redline) = ak47.get_paint_by_name("Redline") {
            println!("🎨 Skin: {}", redline.name);
            println!("💎 Rarity: {}", redline.rarity);
            println!("🏭 Collection: {}", redline.collection.as_deref().unwrap_or("Unknown"));
            println!("📈 Float range: {:.3} - {:.3}", redline.min, redline.max);
            println!("⚡ Has StatTrak: {}", redline.has_stattrak());
            
            println!("\n💰 Normal Prices:");
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
            
            if redline.has_stattrak() {
                println!("\n⚡ StatTrak Prices:");
                for wear in [WearCondition::FactoryNew, WearCondition::MinimalWear, 
                            WearCondition::FieldTested, WearCondition::WellWorn, 
                            WearCondition::BattleScarred] {
                    if let Some(price) = redline.stattrak_price_for_wear(wear) {
                        if let Some(volume) = redline.stattrak_volume_for_wear(wear) {
                            println!("  {} - ${:.2} (Volume: {})", 
                                wear.as_str(), price as f64 / 100.0, volume);
                        }
                    }
                }
            }
        }
    }
    
    // Example 3: Find the most expensive skin
    println!("\n=== Most Expensive Skins ===");
    let mut expensive_skins: Vec<(&str, &str, u64)> = Vec::new();
    
    for (_, weapon) in &schema.weapons {
        for paint in weapon.paints.values() {
            if let Some(price) = paint.price_for_wear(WearCondition::FactoryNew) {
                if price > 0 {
                    expensive_skins.push((&weapon.name, &paint.name, price));
                }
            }
        }
    }
    
    expensive_skins.sort_by(|a, b| b.2.cmp(&a.2));
    
    println!("🏆 Top 10 most expensive Factory New skins:");
    for (i, (weapon, skin, price)) in expensive_skins.iter().take(10).enumerate() {
        println!("  {}. {} | {} - ${:.2}", 
            i + 1, weapon, skin, *price as f64 / 100.0);
    }
    
    // Example 4: Show collections
    println!("\n=== Collections ===");
    println!("📦 Available collections (first 10):");
    for collection in schema.collections.iter().take(10) {
        let souvenir = if collection.has_souvenir.unwrap_or(false) { " (Souvenir)" } else { "" };
        let crate_info = if collection.has_crate.unwrap_or(false) { " (Crate)" } else { "" };
        println!("  • {}{}{}", collection.name, souvenir, crate_info);
    }
    
    println!("\n✅ Schema example completed!");
    
    Ok(())
}