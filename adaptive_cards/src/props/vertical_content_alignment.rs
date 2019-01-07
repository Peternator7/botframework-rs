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
    pub fn is_top(align: &VerticalConentAlignment) -> bool {
        match align {
            VerticalConentAlignment::Top =>  true,
            _ => false,
        }
    }

    pub fn is_center(align: &VerticalConentAlignment) -> bool {
        match align {
            VerticalConentAlignment::Center =>  true,
            _ => false,
        }
    }

    pub fn is_bottom(align: &VerticalConentAlignment) -> bool {
        match align {
            VerticalConentAlignment::Bottom =>  true,
            _ => false,
        }
    }
}