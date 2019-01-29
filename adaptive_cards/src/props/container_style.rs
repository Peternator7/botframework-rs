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
    pub fn is_default(&self) -> bool {
        match self {
            ContainerStyle::Default =>  true,
            _ => false,
        }
    }

    pub fn is_emphasis(&self) -> bool {
        match self {
            ContainerStyle::Emphasis =>  true,
            _ => false,
        }
    }
}