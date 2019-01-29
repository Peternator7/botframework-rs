use std::default::Default;
use serde_derive::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Color {
    Default,
    Dark,
    Light,
    Accent,
    Good,
    Warning,
    Attention,
}

impl Default for Color {
    fn default() -> Color {
        Color::Default
    }
}

impl Color {
    pub fn is_default(&self) -> bool {
        match self {
            Color::Default => true,
            _ => false,
        }
    }

    pub fn is_dark(&self) -> bool {
        match self {
            Color::Dark => true,
            _ => false,
        }
    }

    pub fn is_light(&self) -> bool {
        match self {
            Color::Light => true,
            _ => false,
        }
    }

    pub fn is_accent(&self) -> bool {
        match self {
            Color::Accent => true,
            _ => false,
        }
    }

    pub fn is_warning(&self) -> bool {
        match self {
            Color::Warning => true,
            _ => false,
        }
    }

    pub fn is_good(&self) -> bool {
        match self {
            Color::Good => true,
            _ => false,
        }
    }

    pub fn is_attention(&self) -> bool {
        match self {
            Color::Attention => true,
            _ => false,
        }
    }
}
