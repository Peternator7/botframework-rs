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
    pub fn is_small(size: &ImageSize) -> bool {
        match size {
            ImageSize::Small => true,
            _ => false
        }
    }

    pub fn is_medium(size: &ImageSize) -> bool {
        match size {
            ImageSize::Medium => true,
            _ => false
        }
    }

    pub fn is_large(size: &ImageSize) -> bool {
        match size {
            ImageSize::Large => true,
            _ => false
        }
    }

    pub fn is_auto(size: &ImageSize) -> bool {
        match size {
            ImageSize::Auto => true,
            _ => false
        }
    }

    pub fn is_stretch(size: &ImageSize) -> bool {
        match size {
            ImageSize::Stretch => true,
            _ => false
        }
    }
}