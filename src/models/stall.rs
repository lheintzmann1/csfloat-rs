use serde::{Deserialize, Serialize};
use super::Listing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stall {
    pub data: Vec<Listing>,
}

impl Stall {
    pub fn listings(self) -> Vec<Listing> {
        self.data
    }
}
