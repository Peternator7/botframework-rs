use serde_derive::{Serialize, Deserialize};

pub mod text_block;
pub mod image;
pub mod media;
pub mod container;
pub mod fact_set;
pub mod image_set;
pub mod column_set;
pub mod input;

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
    InputText,
    #[serde(rename = "Input.Number")]
    InputNumber,
    #[serde(rename = "Input.Date")]
    InputDate,
    #[serde(rename = "Input.Time")]
    InputTime,
    #[serde(rename = "Input.Toggle")]
    InputToggle,
    #[serde(rename = "Input.ChoiceSet")]
    InputChoiceSet,
}