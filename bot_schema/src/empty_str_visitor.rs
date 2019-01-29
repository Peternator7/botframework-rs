use std::fmt;

use serde::de::{self, Visitor};

pub struct EmptyStringVisitor;

impl<'de> Visitor<'de> for EmptyStringVisitor {
    type Value = Option<String>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string that may or may not be empty")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where E: de::Error,
    {
        if s.is_empty() {
            Ok(None)
        } else {
            Ok(Some(s.into()))
        }
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
        where E: de::Error
    {
        Ok(None)
    }
}

pub fn deserialize_str <'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where D: de::Deserializer<'de>
{
    deserializer.deserialize_any(EmptyStringVisitor)
}