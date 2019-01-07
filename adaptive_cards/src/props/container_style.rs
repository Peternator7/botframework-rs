use serde_derive::{Serialize, Deserialize};
use std::default::Default;

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ContainerStyle {
    Default,
    Emphasis,
}

impl Default for ContainerStyle {
    fn default() -> ContainerStyle {
        ContainerStyle::Default
    }
}

impl ContainerStyle {
    pub fn is_default(style: &ContainerStyle) -> bool {
        match style {
            ContainerStyle::Default =>  true,
            _ => false,
        }
    }

    pub fn is_emphasis(style: &ContainerStyle) -> bool {
        match style {
            ContainerStyle::Emphasis =>  true,
            _ => false,
        }
    }
}