use std::default::Default;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TextSize {
    Small,
    Default,
    Medium,
    Large,
    ExtraLarge
}

impl Default for TextSize {
    fn default() -> TextSize {
        TextSize::Default
    }
}

impl TextSize {
    pub fn is_small(size: &TextSize) -> bool {
        match size {
            TextSize::Small => true,
            _ => false
        }
    }

    pub fn is_default(size: &TextSize) -> bool {
        match size {
            TextSize::Default => true,
            _ => false
        }
    }

    pub fn is_medium(size: &TextSize) -> bool {
        match size {
            TextSize::Medium => true,
            _ => false
        }
    }

    pub fn is_large(size: &TextSize) -> bool {
        match size {
            TextSize::Large => true,
            _ => false
        }
    }

    pub fn is_extra_large(size: &TextSize) -> bool {
        match size {
            TextSize::ExtraLarge => true,
            _ => false
        }
    }
}