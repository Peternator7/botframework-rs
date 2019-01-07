use serde_derive::{Serialize,Deserialize};

pub mod props;
pub mod elements;
pub mod actions;

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
    pub actions: Vec<crate::actions::Action>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub body: Vec<elements::Element>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub select_action: Option<crate::actions::Action>,
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
