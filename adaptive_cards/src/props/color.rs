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
    pub fn is_default(color: &Color) -> bool {
        match color {
            Color::Default => true,
            _ => false,
        }
    }

    pub fn is_dark(color: &Color) -> bool {
        match color {
            Color::Dark => true,
            _ => false,
        }
    }

    pub fn is_light(color: &Color) -> bool {
        match color {
            Color::Light => true,
            _ => false,
        }
    }

    pub fn is_accent(color: &Color) -> bool {
        match color {
            Color::Accent => true,
            _ => false,
        }
    }

    pub fn is_warning(color: &Color) -> bool {
        match color {
            Color::Warning => true,
            _ => false,
        }
    }

    pub fn is_good(color: &Color) -> bool {
        match color {
            Color::Good => true,
            _ => false,
        }
    }

    pub fn is_attention(color: &Color) -> bool {
        match color {
            Color::Attention => true,
            _ => false,
        }
    }
}
