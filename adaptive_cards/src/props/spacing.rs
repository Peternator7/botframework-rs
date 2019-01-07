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
    pub fn is_default(spacing: &Spacing) -> bool {
        match spacing {
            Spacing::Default => true,
            _ => false,
        }
    }

    pub fn is_none(spacing: &Spacing) -> bool {
        match spacing {
            Spacing::None => true,
            _ => false,
        }
    }

    pub fn is_small(spacing: &Spacing) -> bool {
        match spacing {
            Spacing::Small => true,
            _ => false,
        }
    }

    pub fn is_medium(spacing: &Spacing) -> bool {
        match spacing {
            Spacing::Medium => true,
            _ => false,
        }
    }

    pub fn is_large(spacing: &Spacing) -> bool {
        match spacing {
            Spacing::Large => true,
            _ => false,
        }
    }

    pub fn is_extra_large(spacing: &Spacing) -> bool {
        match spacing {
            Spacing::ExtraLarge => true,
            _ => false,
        }
    }

    pub fn is_padding(spacing: &Spacing) -> bool {
        match spacing {
            Spacing::Padding => true,
            _ => false,
        }
    }
}