use crate::props;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextBlock {

    #[serde(skip_serializing_if = "props::Color::is_default", default)]
    pub color: props::Color,

    #[serde(skip_serializing_if = "props::HorizontalAlignment::is_left", default)]
    pub horizontal_alignment: props::HorizontalAlignment,

    #[serde(default)]
    pub is_subtle: bool,

    #[serde(default)]
    pub max_lines: Option<usize>,

    #[serde(skip_serializing_if = "props::TextSize::is_default", default)]
    pub size: props::TextSize,

    #[serde(default)]
    pub text: String,

    #[serde(skip_serializing_if = "props::Weight::is_default", default)]
    pub weight: props::Weight,

    #[serde(default)]
    pub wrap: bool,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "props::Spacing::is_default", default)]
    pub spacing: props::Spacing,

    #[serde(default)]
    pub seperator: bool,
}