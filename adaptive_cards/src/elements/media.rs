

use crate::props;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Media {

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub sources: Vec<MediaSource>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub poster: Option<String>,

    #[serde(default)]
    pub alt_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "props::Spacing::is_default", default)]
    pub spacing: props::Spacing,

    #[serde(default)]
    pub seperator: bool,
}

fn print_media_source_type<S>(_:&(), serializer:S) -> Result<S::Ok, S::Error>
    where S: serde::ser::Serializer
{
    serializer.serialize_str("MediaSource")
}

#[derive(Serialize, Deserialize, Clone, Default, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MediaSource {
    #[serde(rename="type", serialize_with = "print_media_source_type", skip_deserializing)]
    _type: (),

    #[serde(default)]
    pub mime_type: String,

    #[serde(default)]
    pub url: String
}

impl MediaSource {
    pub fn new (mime_type: &str, url: &str) -> MediaSource {
        MediaSource {
            mime_type: mime_type.into(),
            url: url.into(),
            _type: ()
        }
    }
}