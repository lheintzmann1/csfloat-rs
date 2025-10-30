//! Schema models for CSFloat's undocumented `/schema` endpoint
//! 
//! This endpoint provides weapon pricing data including average prices for different
//! wear conditions, volume data, and information about StatTrak and Souvenir variants.
//! 
//! **Note**: This is an undocumented endpoint and may change without notice.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Response from the /schema endpoint containing weapon pricing data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaResponse {
    pub agents: Option<serde_json::Value>,
    pub collectibles: Option<serde_json::Value>,
    pub collections: Vec<Collection>,
    pub containers: Option<serde_json::Value>,
    pub custom_stickers: Option<serde_json::Value>,
    pub highlight_reels: Option<serde_json::Value>,
    pub keychains: Option<serde_json::Value>,
    pub music_kits: Option<serde_json::Value>,
    pub rarities: Option<serde_json::Value>,
    pub stickers: Option<serde_json::Value>,
    pub weapons: HashMap<String, Weapon>,
}

/// Collection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub key: String,
    pub name: String,
    pub has_souvenir: Option<bool>,
    pub has_crate: Option<bool>,
}

/// Weapon information with paints and pricing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
    pub name: String,
    pub sticker_amount: u32,
    #[serde(rename = "type")]
    pub weapon_type: String,
    pub faction: Option<String>,
    pub paints: HashMap<String, Paint>,
}

/// Paint/skin information with pricing data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paint {
    pub index: u32,
    pub max: f64,
    pub min: f64,
    pub rarity: u32,
    pub name: String,
    pub collection: Option<String>,
    pub image: String,
    pub stattrak: Option<bool>,
    pub souvenir: Option<bool>,
    /// Normal prices by wear condition [Factory New, Minimal Wear, Field-Tested, Well-Worn, Battle-Scarred]
    pub normal_prices: Vec<u64>,
    /// Normal volumes by wear condition [Factory New, Minimal Wear, Field-Tested, Well-Worn, Battle-Scarred]
    pub normal_volume: Vec<u64>,
    /// StatTrak prices by wear condition (if applicable)
    pub stattrak_prices: Option<Vec<u64>>,
    /// StatTrak volumes by wear condition (if applicable)
    pub stattrak_volume: Option<Vec<u64>>,
}

/// Wear condition index for accessing price arrays
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WearCondition {
    FactoryNew = 0,
    MinimalWear = 1,
    FieldTested = 2,
    WellWorn = 3,
    BattleScarred = 4,
}

impl WearCondition {
    /// Get the index for this wear condition in price arrays
    pub fn index(self) -> usize {
        self as usize
    }

    /// Get the string representation of this wear condition
    pub fn as_str(self) -> &'static str {
        match self {
            WearCondition::FactoryNew => "Factory New",
            WearCondition::MinimalWear => "Minimal Wear",
            WearCondition::FieldTested => "Field-Tested",
            WearCondition::WellWorn => "Well-Worn",
            WearCondition::BattleScarred => "Battle-Scarred",
        }
    }
}

impl Paint {
    /// Get the price for a specific wear condition (normal skin)
    pub fn price_for_wear(&self, wear: WearCondition) -> Option<u64> {
        self.normal_prices.get(wear.index()).copied()
    }

    /// Get the volume for a specific wear condition (normal skin)
    pub fn volume_for_wear(&self, wear: WearCondition) -> Option<u64> {
        self.normal_volume.get(wear.index()).copied()
    }

    /// Get the StatTrak price for a specific wear condition
    pub fn stattrak_price_for_wear(&self, wear: WearCondition) -> Option<u64> {
        self.stattrak_prices
            .as_ref()
            .and_then(|prices| prices.get(wear.index()).copied())
    }

    /// Get the StatTrak volume for a specific wear condition
    pub fn stattrak_volume_for_wear(&self, wear: WearCondition) -> Option<u64> {
        self.stattrak_volume
            .as_ref()
            .and_then(|volumes| volumes.get(wear.index()).copied())
    }

    /// Check if this paint has StatTrak variants
    pub fn has_stattrak(&self) -> bool {
        self.stattrak.unwrap_or(false)
    }

    /// Check if this paint has Souvenir variants
    pub fn has_souvenir(&self) -> bool {
        self.souvenir.unwrap_or(false)
    }

    /// Get the float range for this paint
    pub fn float_range(&self) -> (f64, f64) {
        (self.min, self.max)
    }
}

impl Weapon {
    /// Get a paint by its index
    pub fn get_paint_by_index(&self, index: u32) -> Option<&Paint> {
        self.paints.values().find(|paint| paint.index == index)
    }

    /// Get a paint by its name
    pub fn get_paint_by_name(&self, name: &str) -> Option<&Paint> {
        self.paints
            .values()
            .find(|paint| paint.name.to_lowercase() == name.to_lowercase())
    }

    /// Get all paints for this weapon sorted by rarity (highest first)
    pub fn paints_by_rarity(&self) -> Vec<&Paint> {
        let mut paints: Vec<&Paint> = self.paints.values().collect();
        paints.sort_by(|a, b| b.rarity.cmp(&a.rarity));
        paints
    }
}

impl SchemaResponse {
    /// Get a weapon by its ID
    pub fn get_weapon(&self, weapon_id: &str) -> Option<&Weapon> {
        self.weapons.get(weapon_id)
    }

    /// Get a weapon by its name
    pub fn get_weapon_by_name(&self, name: &str) -> Option<(String, &Weapon)> {
        self.weapons
            .iter()
            .find(|(_, weapon)| weapon.name.to_lowercase() == name.to_lowercase())
            .map(|(id, weapon)| (id.clone(), weapon))
    }

    /// Get all weapons sorted by name
    pub fn weapons_by_name(&self) -> Vec<(String, &Weapon)> {
        let mut weapons: Vec<(String, &Weapon)> = 
            self.weapons.iter().map(|(id, weapon)| (id.clone(), weapon)).collect();
        weapons.sort_by(|a, b| a.1.name.cmp(&b.1.name));
        weapons
    }

    /// Find a specific paint across all weapons
    pub fn find_paint(&self, weapon_name: &str, paint_name: &str) -> Option<(&Weapon, &Paint)> {
        self.get_weapon_by_name(weapon_name)
            .and_then(|(_, weapon)| {
                weapon.get_paint_by_name(paint_name)
                    .map(|paint| (weapon, paint))
            })
    }
}