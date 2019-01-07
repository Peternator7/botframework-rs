use std::default::Default;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Height {
    Automatic,
    Stretch,
}

impl Default for Height {
    fn default() -> Height {
        Height::Automatic
    }
}

impl Height {
    pub fn is_automatic(height: &Height) -> bool {
        if let Height::Automatic = height {
            true
        } else {
            false
        }
    }

    pub fn is_stretch(height: &Height) -> bool {
        if let Height::Stretch = height {
            true
        } else {
            false
        }
    }
}