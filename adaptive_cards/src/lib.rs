use serde_derive::{Serialize,Deserialize};

fn print_adaptive_card_type<S>(_:&(), serializer:S) -> Result<S::Ok, S::Error>
    where S: serde::ser::Serializer
{
    serializer.serialize_str("AdaptiveCard")
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdaptiveCard {
    #[serde(rename="type", serialize_with = "print_adaptive_card_type", skip_deserializing)]
    _type: (),
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub actions: Vec<Action>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub body: Vec<Element>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub select_action: Option<Action>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub fallback_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub background_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub speak: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub lang: Option<String>,
}

#[derive(Serialize,Deserialize,Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Element {
    TextBlock(TextBlock),
    Image,
    Media,
    MediaSource,
    Container,
    ColumnSet,
    FactSet,
    ImageSet,
    InputText,
    InputNumber,
    InputDate,
    InputTime,
    InputToggle,
    InputChoiceSet,
}

fn left() -> String {
    return "left".into()
}

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

impl std::default::Default for HorizontalAlignment {
    fn default() -> HorizontalAlignment {
        HorizontalAlignment::Left
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Weight {
    Lighter,
    Default,
    Bolder,
}

impl std::default::Default for Weight {
    fn default() -> Weight {
        Weight::Default
    }
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextBlock {
    color: Option<String>,
    #[serde(default)]
    horizontal_alignment: HorizontalAlignment,
    is_subtle: bool,
    max_lines: Option<usize>,
}

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
        card: Box<AdaptiveCard>,
    },
}

#[cfg(test)]
mod tests {
    use serde_json;
    use super::*;

    #[test]
    fn it_works() {
        let a = AdaptiveCard::default();

        println!("{}", serde_json::to_string(&a).unwrap());
    }
}
