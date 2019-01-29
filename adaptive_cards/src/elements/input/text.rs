use serde_derive::{Serialize, Deserialize};
use crate::props;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TextStyle {
    Text,
    Url,
    #[serde(rename = "tel")]
    Telephone,
    Email
}

impl std::default::Default for TextStyle {
    fn default() -> Self {
        TextStyle::Text
    }
}

impl TextStyle {
    pub fn is_text(&self) -> bool {
        match self {
            TextStyle::Text => true,
            _ => false
        }
    }

    pub fn is_url(&self) -> bool {
        match self {
            TextStyle::Url => true,
            _ => false
        }
    }

    pub fn is_email(&self) -> bool {
        match self {
            TextStyle::Email => true,
            _ => false
        }
    }

    pub fn is_telephone(&self) -> bool {
        match self {
            TextStyle::Telephone => true,
            _ => false
        }
    }
}

#[derive(Serialize,Deserialize, Clone, Eq, PartialEq, Default, Debug)]
pub struct InputText {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_lines: Option<usize>,

    #[serde(default)]
    pub is_multiline: bool,

    #[serde(default, skip_serializing_if="Option::is_none")]
    pub placeholder: Option<String>,

    #[serde(default)]
    pub style: TextStyle,

    #[serde(default, skip_serializing_if="Option::is_none")]
    pub value: Option<String>,

    #[serde(default)]
    pub id: String,

    #[serde(skip_serializing_if = "props::Spacing::is_default", default)]
    pub spacing: props::Spacing,

    #[serde(default)]
    pub seperator: bool,
}