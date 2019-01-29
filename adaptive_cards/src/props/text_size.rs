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
    pub fn is_small(&self) -> bool {
        match self {
            TextSize::Small => true,
            _ => false
        }
    }

    pub fn is_default(&self) -> bool {
        match self {
            TextSize::Default => true,
            _ => false
        }
    }

    pub fn is_medium(&self) -> bool {
        match self {
            TextSize::Medium => true,
            _ => false
        }
    }

    pub fn is_large(&self) -> bool {
        match self {
            TextSize::Large => true,
            _ => false
        }
    }

    pub fn is_extra_large(&self) -> bool {
        match self {
            TextSize::ExtraLarge => true,
            _ => false
        }
    }
}