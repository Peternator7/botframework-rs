use crate::{Jwt, JwtHeaders};

use serde::ser::{Serialize, Serializer, SerializeMap};

impl Serialize for Jwt {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        // We serialize the claims fields.
        let mut m = ser.serialize_map(None)?;
        if let Some(ref iss) = self.issuer {
            m.serialize_entry("iss", iss)?;
        }

        // The audience claim is special. If there is one entry, then we can just serialize a string
        // If there are multiple then we need to serialize them as an array.
        match self.audience.len() {
            0 => {},
            1 => m.serialize_entry("aud", &self.audience.first().unwrap())?,
            _ => m.serialize_entry("aud", &self.audience)?,
        }

        if let Some(ref sub) = self.subject {
            m.serialize_entry("sub", sub)?;
        }

        if let Some(ref jti) = self.jwt_id {
            m.serialize_entry("jti", jti)?;
        }

        if let Some(ref nbf) = self.not_before {
            m.serialize_entry("nbf", &nbf.timestamp())?;
        }

        if let Some(ref iat) = self.issued_at {
            m.serialize_entry("iat", &iat.timestamp())?;
        }

        if let Some(ref exp) = self.expires {
            m.serialize_entry("exp", &exp.timestamp())?;
        }

        if self.scopes.len() > 0 {
            m.serialize_entry("scope", &self.scopes)?;
        }

        // Then anything else the user has added.
        for (k,v) in &self.claims {
            m.serialize_entry(&k, v)?;
        }

        m.end()
    }
}

impl <'a> Serialize for JwtHeaders<'a> {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use std::convert::AsRef;

        // We serialize the claims fields.
        let mut m = ser.serialize_map(None)?;
        m.serialize_entry("typ", "JWT")?;
        m.serialize_entry("alg", self.alg.as_ref())?;

        // Then anything else the user has added.
        for (k,v) in self.headers {
            m.serialize_entry(&k, v)?;
        }

        m.end()
    }
}

#[cfg(test)]
mod tests {
    use crate::alg::Algorithm;
    use crate::{Jwt};

    use std::collections::HashMap;
    use chrono::offset::{TimeZone, Utc};

    #[test]
    fn serialize_headers() {
        let map = vec![
            ("kid".to_string(), "abcd".to_string()),
        ].into_iter().collect::<HashMap<_,_>>();

        let headers = crate::JwtHeaders {
            alg: crate::alg::Algorithm::HS256,
            headers: &map,
        };

        let expected = r#"{"typ":"JWT","alg":"HS256","kid":"abcd"}"#;
        let actual = serde_json::to_string(&headers).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn serialize_jwt() {
        let jwt = Jwt {
            alg: Algorithm::HS256,
            headers: HashMap::new(),
            issuer: Some("Issuer".to_string()),
            subject: Some("Subject".to_string()),
            audience: vec!["Audience".to_string()],
            not_before: Some(Utc.ymd(2019,1,1).and_hms(0,0,0)),
            expires: Some(Utc.ymd(2019,1,1).and_hms(1,0,0)),
            issued_at: Some(Utc.ymd(2018,12,31).and_hms(0,0,0)),
            jwt_id: Some("JwtId".to_string()),
            claims: vec![
                ("claim".to_string(), json!(false)),
            ].into_iter().collect::<HashMap<_,_>>(),
            scopes: vec![
                "Scope1".to_string(),
                "Scope2".to_string()
            ],
        };

        let actual = serde_json::to_string(&jwt).unwrap();
        let expected =
        "{\
            \"iss\":\"Issuer\",\
            \"aud\":\"Audience\",\
            \"sub\":\"Subject\",\
            \"jti\":\"JwtId\",\
            \"nbf\":1546300800,\
            \"iat\":1546214400,\
            \"exp\":1546304400,\
            \"scope\":[\"Scope1\",\"Scope2\"],\
            \"claim\":false\
        }";

        assert_eq!(expected, actual);
    }

    #[test]
    fn serialize_jwt_multiple_audiences() {
        let jwt = Jwt {
            alg: Algorithm::HS256,
            headers: HashMap::new(),
            issuer: Some("Issuer".to_string()),
            subject: Some("Subject".to_string()),
            audience: vec!["Audience".to_string(), "Audience-2".to_string()],
            ..std::default::Default::default()
        };

        let actual = serde_json::to_string(&jwt).unwrap();
        let expected =
        "{\
            \"iss\":\"Issuer\",\
            \"aud\":[\"Audience\",\"Audience-2\"],\
            \"sub\":\"Subject\"\
        }";

        assert_eq!(expected, actual);
    }
}