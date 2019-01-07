use crate::props;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FactSet {

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub facts: Vec<Fact>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "props::Spacing::is_default", default)]
    pub spacing: props::Spacing,

    #[serde(default)]
    pub seperator: bool,
}

fn print_fact_type<S>(_:&(), serializer:S) -> Result<S::Ok, S::Error>
    where S: serde::ser::Serializer
{
    serializer.serialize_str("Fact")
}

#[derive(Serialize, Deserialize, Clone, Default, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Fact {
    #[serde(rename="type", serialize_with = "print_fact_type", skip_deserializing)]
    _type: (),

    #[serde(default)]
    pub title: String,

    #[serde(default)]
    pub value: String,
}

impl Fact {
    pub fn new (title: &str, value: &str) -> Fact {
        Fact {
            title: title.into(),
            value: value.into(),
            _type: ()
        }
    }
}