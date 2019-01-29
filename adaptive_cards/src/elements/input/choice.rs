use serde_derive::{Serialize, Deserialize};
use crate::props;

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ChoiceStyle {
    Compact,
    Expanded
}

impl std::default::Default for ChoiceStyle {
    fn default() -> ChoiceStyle {
        ChoiceStyle::Compact
    }
}

impl ChoiceStyle {
    pub fn is_compact(&self) -> bool {
        match self {
            ChoiceStyle::Compact => true,
            _ => false,
        }
    }

    pub fn is_expanded(&self) -> bool {
        match self {
            ChoiceStyle::Expanded => true,
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InputChoiceSet {

    #[serde(default)]
    pub choices: Vec<Choice>,

    #[serde(default)]
    pub is_multi_select: bool,

    #[serde(default)]
    pub style: ChoiceStyle,

    #[serde(default, skip_serializing_if="Option::is_none")]
    pub value: Option<String>,

    #[serde(default)]
    pub id: String,

    #[serde(skip_serializing_if = "props::Spacing::is_default", default)]
    pub spacing: props::Spacing,

    #[serde(default)]
    pub seperator: bool,
}

#[derive(Serialize, Deserialize, Default, Debug, Eq, PartialEq, Clone)]
pub struct Choice {
    pub title: String,
    pub value: String,
}

impl Choice {
    pub fn new(title: String, value: String) -> Choice {
        Choice { title, value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
    }
}