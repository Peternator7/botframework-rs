use serde_derive::{Serialize, Deserialize};
use crate::props;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InputToggle {
    #[serde(default)]
    pub title: String,

    #[serde(default, skip_serializing_if="Option::is_none")]
    pub placeholder: Option<String>,

    #[serde(default, skip_serializing_if="Option::is_none")]
    pub value: Option<String>,

    #[serde(default, skip_serializing_if="Option::is_none")]
    pub value_off: Option<String>,

    #[serde(default, skip_serializing_if="Option::is_none")]
    pub value_on: Option<String>,

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
    }
}