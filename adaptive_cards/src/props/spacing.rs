use std::default::Default;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Spacing {
    None,
    Small,
    Default,
    Medium,
    Large,
    ExtraLarge,
    Padding,
}

impl Default for Spacing {
    fn default() -> Spacing {
        Spacing::Default
    }
}

impl Spacing {
    pub fn is_default(&self) -> bool {
        match self {
            Spacing::Default => true,
            _ => false,
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            Spacing::None => true,
            _ => false,
        }
    }

    pub fn is_small(&self) -> bool {
        match self {
            Spacing::Small => true,
            _ => false,
        }
    }

    pub fn is_medium(&self) -> bool {
        match self {
            Spacing::Medium => true,
            _ => false,
        }
    }

    pub fn is_large(&self) -> bool {
        match self {
            Spacing::Large => true,
            _ => false,
        }
    }

    pub fn is_extra_large(&self) -> bool {
        match self {
            Spacing::ExtraLarge => true,
            _ => false,
        }
    }

    pub fn is_padding(&self) -> bool {
        match self {
            Spacing::Padding => true,
            _ => false,
        }
    }
}