use crate::props;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ColumnSet {

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub columns: Vec<Column>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub select_action: Option<crate::actions::Action>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "props::Spacing::is_default", default)]
    pub spacing: props::Spacing,

    #[serde(default)]
    pub seperator: bool,
}

fn print_column_type<S>(_:&(), serializer:S) -> Result<S::Ok, S::Error>
    where S: serde::ser::Serializer
{
    serializer.serialize_str("Column")
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Column {

    #[serde(rename="type", serialize_with = "print_column_type", skip_deserializing)]
    _type: (),

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub items: Vec<super::Element>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub select_action: Option<crate::actions::Action>,

    #[serde(skip_serializing_if = "props::ContainerStyle::is_default", default)]
    pub style: props::ContainerStyle,

    #[serde(skip_serializing_if = "props::WidthOrNumber::is_automatic", default)]
    pub width: props::WidthOrNumber,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "props::Spacing::is_default", default)]
    pub spacing: props::Spacing,

    #[serde(default)]
    pub seperator: bool,
}