use crate::props;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImageSet {

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub images: Vec<super::image::Image>,

    #[serde(skip_serializing_if = "props::ImageSize::is_auto", default)]
    pub size: props::ImageSize,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "props::Spacing::is_default", default)]
    pub spacing: props::Spacing,

    #[serde(default)]
    pub seperator: bool,
}