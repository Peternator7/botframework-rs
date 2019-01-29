use serde_derive::{Serialize, Deserialize};
use crate::props;

#[derive(Serialize,Deserialize, Clone, PartialEq, Default, Debug)]
pub struct InputDate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<chrono::NaiveDate>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<chrono::NaiveDate>,

    #[serde(default, skip_serializing_if="Option::is_none")]
    pub placeholder: Option<String>,

    #[serde(default, skip_serializing_if="Option::is_none")]
    pub value: Option<chrono::NaiveDate>,

    #[serde(default)]
    pub id: String,

    #[serde(skip_serializing_if = "props::Spacing::is_default", default)]
    pub spacing: props::Spacing,

    #[serde(default)]
    pub seperator: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let a = InputDate {
            max: chrono::NaiveDate::from_ymd_opt(2019, 12, 1),
            spacing: crate::props::Spacing::ExtraLarge,
            ..InputDate::default()
        };

        println!("{}", serde_json::to_string(&a).unwrap());
    }
}