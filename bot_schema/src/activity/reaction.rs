// External Dependencies
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize,Deserialize, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct MessageReaction {
    #[serde(rename = "type")]
    pub ty: String,
}
