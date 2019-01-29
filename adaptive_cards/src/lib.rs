/// A rust port of the [Adaptive Card](https://adaptivecards.io) library developed by Microsoft.
use serde_derive::{Serialize,Deserialize};

pub mod props;
pub mod elements;
pub mod actions;

fn print_adaptive_card_type<S>(_:&(), serializer:S) -> Result<S::Ok, S::Error>
    where S: serde::ser::Serializer
{
    serializer.serialize_str("AdaptiveCard")
}

fn print_adaptive_card_schema<S>(_:&(), serializer:S) -> Result<S::Ok, S::Error>
    where S: serde::ser::Serializer
{
    serializer.serialize_str("http://adaptivecards.io/schemas/adaptive-card.json")
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdaptiveCard {
    #[serde(rename="type", serialize_with = "print_adaptive_card_type", skip_deserializing)]
    _type: (),

    #[serde(rename="$content", serialize_with = "print_adaptive_card_schema", skip_deserializing)]
    _schema: (),

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

pub struct AdaptiveCardBuilder {
    card: AdaptiveCard,
}

impl AdaptiveCard {
    pub fn builder() -> AdaptiveCardBuilder {
        AdaptiveCardBuilder {
            card: AdaptiveCard::default()
        }
    }
}

impl AdaptiveCardBuilder {
    pub fn action(mut self, action: crate::actions::Action) -> Self  {
        self.card.actions.push(action);
        self
    }

    pub fn select_action(mut self, action: crate::actions::Action) -> Self {
        self.card.select_action = Some(action);
        self
    }

    pub fn element<E>(mut self, element: E) -> Self
        where E: Into<crate::elements::Element>
    {
        self.card.body.push(element.into());
        self
    }

    pub fn version<S>(mut self, version: S) -> Self
        where S: Into<String>
    {
        self.card.version = Some(version.into());
        self
    }

    pub fn fallback_text<S>(mut self, text: S) -> Self
        where S: Into<String>
    {
        self.card.fallback_text = Some(text.into());
        self
    }

    pub fn background_image<S>(mut self, url: S) -> Self
        where S: Into<String>
    {
        self.card.background_image = Some(url.into());
        self
    }

    pub fn speak<S>(mut self, text: S) -> Self
        where S: Into<String>
    {
        self.card.speak = Some(text.into());
        self
    }

    pub fn lang<S>(mut self, lang: S) -> Self
        where S: Into<String>
    {
        self.card.lang = Some(lang.into());
        self
    }
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
