use crate::attachments::Attachment;

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize,Deserialize, Clone, Copy, Eq, PartialEq, Hash, Debug)]
#[serde(rename_all="lowercase")]
pub enum TextFormat {
    Plain,
    Xml,
    Markdown,
}

impl std::default::Default for TextFormat {
    fn default() -> Self {
        TextFormat::Plain
    }
}

impl TextFormat {
    pub fn is_plain(&self) -> bool {
        match self {
            TextFormat::Plain => true,
            _ => false,
        }
    }

    pub fn is_xml(&self) -> bool {
        match self {
            TextFormat::Xml => true,
            _ => false,
        }
    }

    pub fn is_markdown(&self) -> bool {
        match self {
            TextFormat::Markdown => true,
            _ => false,
        }
    }
}

#[derive(Serialize,Deserialize, Clone, PartialEq, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageDetails {
    #[serde(default)]
    pub text_format: TextFormat,

    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "crate::empty_str_visitor::deserialize_str", default)]
    pub text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "crate::empty_str_visitor::deserialize_str", default)]
    pub locale: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub attachments: Vec<Attachment>,
}

impl MessageDetails {
    pub fn with_text(text:String) -> MessageDetails {
        MessageDetails {
            text: Some(text),
            ..MessageDetails::default()
        }
    }

    pub fn with_markdown(text:String) -> MessageDetails {
        MessageDetails {
            text: Some(text),
            text_format: TextFormat::Markdown,
            ..MessageDetails::default()
        }
    }

    pub fn with_xml(text: String) -> MessageDetails {
        MessageDetails {
            text: Some(text),
            text_format: TextFormat::Xml,
            ..MessageDetails::default()
        }
    }

    pub fn with_attachments(attachments: Vec<Attachment>) -> MessageDetails {
        MessageDetails {
            attachments: attachments,
            ..MessageDetails::default()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_locale() {
        let a = r#"
        {
            "textFormat": "xml",
            "text":"this-is-text",
            "locale":null
        }
        "#;

        let message: MessageDetails = serde_json::from_str(a).unwrap();
        assert_eq!(None, message.locale);
        assert_eq!(TextFormat::Xml, message.text_format);
        assert_eq!(Some("this-is-text".into()), message.text);
    }

    #[test]
    fn empty_locale() {
        let a = r#"
        {
            "textFormat": "plain",
            "text":"this-is-text",
            "locale":""
        }
        "#;

        let message: MessageDetails = serde_json::from_str(a).unwrap();
        assert_eq!(None, message.locale);
        assert_eq!(TextFormat::Plain, message.text_format);
        assert_eq!(Some("this-is-text".into()), message.text);
    }

    #[test]
    fn missing_locale() {
        let a = r#"
        {
            "textFormat": "plain",
            "text":"this-is-text"
        }
        "#;

        let message: MessageDetails = serde_json::from_str(a).unwrap();
        assert_eq!(None, message.locale);
        assert_eq!(TextFormat::Plain, message.text_format);
        assert_eq!(Some("this-is-text".into()), message.text);
    }
}
