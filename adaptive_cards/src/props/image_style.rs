use std::default::Default;
use serde_derive::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ImageStyle {
    Default,
    Person,
}

impl Default for ImageStyle {
    fn default() -> ImageStyle {
        ImageStyle::Default
    }
}

impl ImageStyle {
    pub fn is_default(&self) -> bool {
        match self {
            ImageStyle::Default => true,
            _ => false,
        }
    }

    pub fn is_person(&self) -> bool {
        match self {
            ImageStyle::Person => true,
            _ => false,
        }
    }
}