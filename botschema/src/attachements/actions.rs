use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(tag="type", rename_all = "camelCase")]
pub enum CardAction {
    MessageBack {
        #[serde(skip_serializing_if = "Option::is_none", default)]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        image: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        display_text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        value: Option<serde_json::Value>,
    },
    ImBack {
        #[serde(skip_serializing_if = "Option::is_none", default)]
        title: Option<String>,
        // #[serde(skip_serializing_if = "Option::is_none", default)]
        // image: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        value: Option<String>,
    },
    PostBack {
        #[serde(skip_serializing_if = "Option::is_none", default)]
        title: Option<String>,
        // #[serde(skip_serializing_if = "Option::is_none", default)]
        // image: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        value: Option<String>,
    },
    OpenUrl {
        #[serde(skip_serializing_if = "Option::is_none", default)]
        title: Option<String>,
        // #[serde(skip_serializing_if = "Option::is_none", default)]
        // image: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        value: Option<String>,
    },
    DownloadFile {
        #[serde(skip_serializing_if = "Option::is_none", default)]
        title: Option<String>,
        // #[serde(skip_serializing_if = "Option::is_none", default)]
        // image: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        value: Option<String>,
    },
    ShowImage {
        #[serde(skip_serializing_if = "Option::is_none", default)]
        title: Option<String>,
        // #[serde(skip_serializing_if = "Option::is_none", default)]
        // image: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        value: Option<String>,
    },
    Signin {
        #[serde(skip_serializing_if = "Option::is_none", default)]
        title: Option<String>,
        // #[serde(skip_serializing_if = "Option::is_none", default)]
        // image: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        value: Option<String>,
    },
    PlayAudio {
        #[serde(skip_serializing_if = "Option::is_none", default)]
        title: Option<String>,
        // #[serde(skip_serializing_if = "Option::is_none", default)]
        // image: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        value: Option<String>,
    },
    PlayVideo {
        #[serde(skip_serializing_if = "Option::is_none", default)]
        title: Option<String>,
        // #[serde(skip_serializing_if = "Option::is_none", default)]
        // image: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        value: Option<String>,
    },
    Call {
        #[serde(skip_serializing_if = "Option::is_none", default)]
        title: Option<String>,
        // #[serde(skip_serializing_if = "Option::is_none", default)]
        // image: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        value: Option<String>,
    },
    Payment {
        #[serde(skip_serializing_if = "Option::is_none", default)]
        title: Option<String>,
        // #[serde(skip_serializing_if = "Option::is_none", default)]
        // image: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        value: Option<serde_json::Value>,
    },
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CardImage {
    pub url: String,
}