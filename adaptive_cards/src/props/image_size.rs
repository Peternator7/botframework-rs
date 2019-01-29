use std::default::Default;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ImageSize {
    Auto,
    Stretch,
    Small,
    Medium,
    Large,
}

impl Default for ImageSize {
    fn default() -> ImageSize {
        ImageSize::Auto
    }
}

impl ImageSize {
    pub fn is_small(&self) -> bool {
        match self {
            ImageSize::Small => true,
            _ => false
        }
    }

    pub fn is_medium(&self) -> bool {
        match self {
            ImageSize::Medium => true,
            _ => false
        }
    }

    pub fn is_large(&self) -> bool {
        match self {
            ImageSize::Large => true,
            _ => false
        }
    }

    pub fn is_auto(&self) -> bool {
        match self {
            ImageSize::Auto => true,
            _ => false
        }
    }

    pub fn is_stretch(&self) -> bool {
        match self {
            ImageSize::Stretch => true,
            _ => false
        }
    }
}