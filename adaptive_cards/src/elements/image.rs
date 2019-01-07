use crate::props;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Image {

    #[serde(default)]
    pub alt_text: Option<String>,

    #[serde(skip_serializing_if = "props::HorizontalAlignment::is_left", default)]
    pub horizontal_alignment: props::HorizontalAlignment,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub select_action: Option<crate::actions::Action>,

    #[serde(skip_serializing_if = "props::ImageSize::is_auto", default)]
    pub size: props::ImageSize,

    #[serde(skip_serializing_if = "props::ImageStyle::is_default", default)]
    pub style: props::ImageStyle,

    #[serde(default)]
    pub url:String,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "props::Spacing::is_default", default)]
    pub spacing: props::Spacing,

    #[serde(default)]
    pub seperator: bool,
}