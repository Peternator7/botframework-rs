use serde_derive::{Serialize, Deserialize};
use std::default::Default;

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum VerticalConentAlignment {
    Top,
    Center,
    Bottom,
}

impl Default for VerticalConentAlignment {
    fn default() -> VerticalConentAlignment {
        VerticalConentAlignment::Top
    }
}

impl VerticalConentAlignment {
    pub fn is_top(&self) -> bool {
        match self {
            VerticalConentAlignment::Top =>  true,
            _ => false,
        }
    }

    pub fn is_center(&self) -> bool {
        match self {
            VerticalConentAlignment::Center =>  true,
            _ => false,
        }
    }

    pub fn is_bottom(&self) -> bool {
        match self {
            VerticalConentAlignment::Bottom =>  true,
            _ => false,
        }
    }
}
