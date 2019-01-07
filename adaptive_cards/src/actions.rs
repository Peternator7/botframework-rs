
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,Clone, PartialEq, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Action {
    #[serde(rename = "Action.OpenUrl")]
    OpenUrl {
        title: String,
        url:String,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        icon_url: Option<String>,
    },
    #[serde(rename = "Action.Submit")]
    Submit {
        title: String,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        icon_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        data: Option<serde_json::Value>,
    },
    #[serde(rename = "Action.ShowCard")]
    ShowCard {
        title: String,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        icon_url: Option<String>,
        card: Box<crate::AdaptiveCard>,
    },
}