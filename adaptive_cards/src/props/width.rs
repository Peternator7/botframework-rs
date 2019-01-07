use std::default::Default;
use serde_derive::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Width {
    Automatic,
    Stretch,
}

impl Default for Width {
    fn default() -> Width {
        Width::Automatic
    }
}

impl Width {
    pub fn is_automatic(width: &Width) -> bool {
        if let Width::Automatic = width {
            true
        } else {
            false
        }
    }

    pub fn is_stretch(width: &Width) -> bool {
        if let Width::Stretch = width {
            true
        } else {
            false
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(untagged)]
pub enum WidthOrNumber {
    Number(serde_json::Number),
    Width(Width),
}

impl Default for WidthOrNumber {
    fn default() -> WidthOrNumber {
        WidthOrNumber::Width(Width::Automatic)
    }
}

impl WidthOrNumber {
    pub fn is_automatic(width: &WidthOrNumber) -> bool {
        match width {
            WidthOrNumber::Width(Width::Automatic) => true,
            _ => false,
        }
    }

    pub fn is_stretch(width: &WidthOrNumber) -> bool {
        match width {
            WidthOrNumber::Width(Width::Stretch) => true,
            _ => false,
        }
    }

    pub fn is_number(width: &WidthOrNumber) -> bool {
        match width {
            WidthOrNumber::Number(_) => true,
            _ => false,
        }
    }
}