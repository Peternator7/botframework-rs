use serde_derive::{Serialize, Deserialize};
use std::default::Default;

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

impl Default for HorizontalAlignment {
    fn default() -> HorizontalAlignment {
        HorizontalAlignment::Left
    }
}

impl HorizontalAlignment {
    pub fn is_left(align: &HorizontalAlignment) -> bool {
        match align {
            HorizontalAlignment::Left =>  true,
            _ => false,
        }
    }

    pub fn is_center(align: &HorizontalAlignment) -> bool {
        match align {
            HorizontalAlignment::Center =>  true,
            _ => false,
        }
    }

    pub fn is_right(align: &HorizontalAlignment) -> bool {
        match align {
            HorizontalAlignment::Right =>  true,
            _ => false,
        }
    }
}