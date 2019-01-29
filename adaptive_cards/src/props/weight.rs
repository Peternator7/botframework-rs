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
    pub fn is_default(&self) -> bool {
        match self {
            Weight::Default => true,
            _ => false,
        }
    }

    pub fn is_lighter(&self) -> bool {
        match self {
            Weight::Lighter => true,
            _ => false,
        }
    }

    pub fn is_bolder(&self) -> bool {
        match self {
            Weight::Bolder => true,
            _ => false,
        }
    }
}
