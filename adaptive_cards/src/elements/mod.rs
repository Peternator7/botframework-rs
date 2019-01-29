use serde_derive::{Serialize, Deserialize};

pub mod text_block;
pub mod image;
pub mod media;
pub mod container;
pub mod fact_set;
pub mod image_set;
pub mod column_set;
pub mod input;

pub use self::text_block::*;
pub use self::image::*;
pub use self::media::*;
pub use self::container::*;
pub use self::fact_set::*;
pub use self::image_set::*;
pub use self::column_set::*;

#[derive(Serialize,Deserialize,Clone, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum Element {
    TextBlock(text_block::TextBlock),

    Image(image::Image),

    Media(media::Media),

    Container(container::Container),

    ColumnSet(column_set::ColumnSet),

    FactSet(fact_set::FactSet),

    ImageSet(image_set::ImageSet),

    #[serde(rename = "Input.Text")]
    InputText(input::text::InputText),

    #[serde(rename = "Input.Number")]
    InputNumber(input::number::InputNumber),

    #[serde(rename = "Input.Date")]
    InputDate(input::date::InputDate),

    #[serde(rename = "Input.Time")]
    InputTime(input::time::InputTime),

    #[serde(rename = "Input.Toggle")]
    InputToggle(input::toggle::InputToggle),

    #[serde(rename = "Input.ChoiceSet")]
    InputChoiceSet(input::choice::InputChoiceSet),
}

impl From<input::choice::InputChoiceSet> for Element {
    fn from(i: input::choice::InputChoiceSet) -> Element {
        Element::InputChoiceSet(i)
    }
}