use crate::props;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Container {

    #[serde(default)]
    pub alt_text: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub items: Vec<super::Element>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub select_action: Option<crate::actions::Action>,

    #[serde(skip_serializing_if = "props::ContainerStyle::is_default", default)]
    pub style: props::ContainerStyle,

    #[serde(skip_serializing_if = "props::VerticalConentAlignment::is_top", default)]
    pub vertical_content_alignment: props::VerticalConentAlignment,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "props::Spacing::is_default", default)]
    pub spacing: props::Spacing,

    #[serde(default)]
    pub seperator: bool,
}