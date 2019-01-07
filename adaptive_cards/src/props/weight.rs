use std::default::Default;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Weight {
    Lighter,
    Default,
    Bolder,
}

impl Default for Weight {
    fn default() -> Weight {
        Weight::Default
    }
}

impl Weight {
    pub fn is_default(weight: &Weight) -> bool {
        if let Weight::Default = weight {
            true
        } else {
            false
        }
    }
}