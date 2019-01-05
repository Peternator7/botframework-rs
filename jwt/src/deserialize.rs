use crate::Jwt;

use chrono::{Utc, TimeZone};
use std::fmt;
use serde_json::{Value, Number};
use serde::de::{Visitor, MapAccess, Deserialize, Deserializer, Unexpected, Error};
use std::string::ToString;

// map a Number to an Unexpected
fn number_to_unexpected(n: &Number) -> Unexpected {
    if let Some(f) = n.as_f64() {
        return Unexpected::Float(f);
    }

    if let Some(i) = n.as_i64() {
        return Unexpected::Signed(i);
    }

    if let Some(u) = n.as_u64() {
        return Unexpected::Unsigned(u);
    }

    Unexpected::Other("Invalid number")
}

// Map a json Value to an Unexpected value.
fn value_to_unexpected(v: &Value) -> Unexpected {
    match *v {
        Value::Null => Unexpected::Unit,
        Value::Bool(b) => Unexpected::Bool(b),
        Value::Number(ref n) => number_to_unexpected(n),
        Value::String(ref s) => Unexpected::Str(&*s),
        Value::Array(_) => Unexpected::Seq,
        Value::Object(_) => Unexpected::Map,
    }
}

struct JwtVisitor;

impl <'de> Visitor<'de> for JwtVisitor {
    type Value = Jwt;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Expecting a JWT Payload object.")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Jwt, A::Error>
        where A: MapAccess<'de>
    {
        let mut output = Jwt::new();
        output.issued_at = None;

        while let Some((key, value)) = map.next_entry::<&'de str, Value>()? {
            match key {
                "aud" => {
                    match value {
                        // Audience should be a string or an array.
                        Value::String(s) => output.audience = vec![s],
                        Value::Array(ref a) => {
                            let aud = a.iter().map(|v| v.as_str().map(ToString::to_string))
                                .collect::<Option<Vec<String>>>();

                            if let Some(aud) = aud {
                                output.audience = aud;
                            } else {
                                return Err(A::Error::invalid_type(value_to_unexpected(&value), &"string"));
                            }
                        },
                        _ => return Err(A::Error::invalid_type(value_to_unexpected(&value), &"string")),
                    }
                },
                "iss" => {
                    if let Value::String(s) = value {
                        output.issuer = Some(s);
                    } else {
                        // Issuer needs to be a string.
                        return Err(A::Error::invalid_type(value_to_unexpected(&value), &"a string"));
                    }
                },
                "sub" => {
                    if let Value::String(s) = value {
                        output.subject = Some(s);
                    } else {
                        // subject needs to be a string.
                        return Err(A::Error::invalid_type(value_to_unexpected(&value), &"a string"));
                    }
                },
                "exp" => {
                    if let Value::Number(num) = value {
                        if let Some(n) = num.as_i64() {
                            output.expires = Some(Utc.timestamp(n,0));
                        } else {
                            return Err(A::Error::invalid_type(number_to_unexpected(&num), &"an integer"));
                        }
                    } else {
                        return Err(A::Error::invalid_type(value_to_unexpected(&value), &"an integer"));
                    }
                },
                "nbf" => {
                    if let Value::Number(num) = value {
                        if let Some(n) = num.as_i64() {
                            output.not_before = Some(Utc.timestamp(n,0));
                        } else {
                            return Err(A::Error::invalid_type(number_to_unexpected(&num), &"an integer"));
                        }
                    } else {
                        return Err(A::Error::invalid_type(value_to_unexpected(&value), &"an integer"));
                    }
                },
                "iat" => {
                    if let Value::Number(num) = value {
                        if let Some(n) = num.as_i64() {
                            output.issued_at = Some(Utc.timestamp(n,0));
                        } else {
                            return Err(A::Error::invalid_type(number_to_unexpected(&num), &"an integer"));
                        }
                    } else {
                        return Err(A::Error::invalid_type(value_to_unexpected(&value), &"an integer"));
                    }
                },
                "jti" => {
                    if let Value::String(s) = value {
                        output.jwt_id = Some(s);
                    } else {
                        return Err(A::Error::invalid_type(value_to_unexpected(&value), &"a string"));
                    }
                },
                "scope" => {
                    match value {
                        Value::String(s) => output.scopes = vec![s],
                        Value::Array(ref a) => {
                            let scopes = a.iter().map(|v| v.as_str().map(ToString::to_string))
                                .collect::<Option<Vec<String>>>();
                            if let Some(scopes) = scopes {
                                output.scopes = scopes;
                            } else {
                                return Err(A::Error::invalid_type(value_to_unexpected(&value), &"an array of strings"));
                            }
                        },
                        _ => return Err(A::Error::invalid_type(value_to_unexpected(&value), &"an array of strings")),
                    }
                },
                _ => {
                    output.claims.insert(key.to_string(), value);
                },
            }
        }

        Ok(output)
    }
}

impl <'de> Deserialize<'de> for Jwt {
    fn deserialize<D>(d: D) -> Result<Jwt, D::Error>
        where D: Deserializer<'de>
    {
        d.deserialize_map(JwtVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::Jwt;
    use crate::alg::Algorithm;

    use std::collections::HashMap;

    use chrono::offset::{TimeZone, Utc};

    #[test]
    fn simple_deserializer() {
        let s = "{\"sub\":\"1234567890\",\"name\":\"John Doe\",\"admin\":true, \"iat\":15 }";
        let c = serde_json::from_str::<Jwt>(s).unwrap();

        assert_eq!(Some("1234567890".into()), c.subject);
        assert_eq!(json!(true), c.claims["admin"]);
        assert_eq!(json!("John Doe"), c.claims["name"]);
        assert_eq!(Some(Utc.ymd(1970, 1, 1).and_hms(0,0,15)), c.issued_at);
    }

    #[test]
    fn audience_test_single() {
        let s = "{\"aud\":\"my-audience\" }";
        let c = serde_json::from_str::<Jwt>(s).unwrap();
        let expected: Vec<String> = vec![
            String::from("my-audience"),
        ];

        assert_eq!(expected, c.audience);
    }

    #[test]
    fn audience_test_array() {
        let s = "{\"aud\":[\"my-audience\", \"second-item\"] }";
        let c = serde_json::from_str::<Jwt>(s).unwrap();
        let expected: Vec<String> = vec![
            String::from("my-audience"),
            String::from("second-item"),
        ];

        assert_eq!(expected, c.audience);
    }

    #[test]
    fn complex_json() {
        let expected = Jwt {
            alg: Algorithm::None,
            headers: HashMap::new(),
            issuer: Some("Issuer".to_string()),
            subject: Some("Subject".to_string()),
            audience: vec!["Audience".to_string()],
            not_before: Some(Utc.ymd(2019,1,1).and_hms(0,0,0)),
            expires: Some(Utc.ymd(2019,1,1).and_hms(1,0,0)),
            issued_at: Some(Utc.ymd(2018,12,31).and_hms(0,0,0)),
            jwt_id: Some("JwtId".to_string()),
            claims: vec![
                ("claim".to_string(), json!("value")),
                ("claim-2".to_string(), json!(false)),
                ("claim-3".to_string(), json!(5)),
            ].into_iter().collect::<HashMap<_,_>>(),
            scopes: vec![
                "Scope1".to_string(),
                "Scope2".to_string()
            ],
        };

        let input =
        "{\
            \"iss\":\"Issuer\",\
            \"aud\":\"Audience\",\
            \"sub\":\"Subject\",\
            \"jti\":\"JwtId\",\
            \"nbf\":1546300800,\
            \"iat\":1546214400,\
            \"exp\":1546304400,\
            \"scope\":[\"Scope1\",\"Scope2\"],\
            \"claim\":\"value\",\
            \"claim-2\":false,\
            \"claim-3\":5\
        }";

        let actual = serde_json::from_str(input).unwrap();
        assert_eq!(expected, actual);
    }
}