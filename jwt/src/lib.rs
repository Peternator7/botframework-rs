#[macro_use]
extern crate strum_macros;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;

mod serialize;
mod deserialize;
mod error;
mod crypto;
mod alg;
mod builder;
mod token;
mod unverified_token;

pub use crate::alg::Algorithm;

pub use serde_json::Value;
use chrono::{Utc, DateTime};
use std::str;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct JwtBuilder {
    jwt: Jwt,
}

///
#[derive(Debug, Clone, PartialEq)]
pub struct Jwt {
    // Headers
    pub alg: Algorithm,
    pub headers: HashMap<String, String>,
    // Payload
    pub issuer: Option<String>,
    pub subject: Option<String>,
    pub audience: Vec<String>,
    pub not_before: Option<DateTime<Utc>>,
    pub expires: Option<DateTime<Utc>>,
    pub issued_at: Option<DateTime<Utc>>,
    pub jwt_id: Option<String>,
    // Custom Fields
    pub claims: HashMap<String, Value>,
    pub scopes: Vec<String>,
}

// This struct isn't exposed via the api. It just exists so that we can serialize the headers
// without having to copy the HashMap out of the Jwt struct.
struct JwtHeaders<'a> {
    headers: &'a HashMap<String,String>,
    alg: Algorithm,
}

#[derive(Debug, Clone)]
pub struct UnverifiedJwt<'a> {
    pub alg: Algorithm,
    pub headers: HashMap<String, String>,
    header_payload: &'a str,
    sig: Option<&'a str>,
    inner: Jwt,
}

#[cfg(test)]
mod tests {
    use crate::Jwt;
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

    //#[test]
    //fn it_works_2() {
    //    let s = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWV9.TJVA95OrM7E2cBab30RMHrHDcEfxjoYZgeFONFh7HgQ";
    //    let (jwt, sig) = Jwt::decode(s).unwrap();
    //    println!("{:?}", jwt);
    //    sig.unwrap().verify("secret".as_bytes()).unwrap();
    //    println!("Verified secret");
    //}
    //
    //#[test]
    //fn encode_test() {
    //    let mut c = Jwt::new();
    //    c.alg = Algorithm::HS256;
    //    c.subject = Some("1234567890".to_string());
    //    c.claims.insert("name".to_string(), Value::String("Peternator7".to_string()));
    //    c.claims.insert("admin".to_string(), Value::Bool(true));
    //    let encoded = c.encode("secret".as_bytes());
    //    println!("Encoded: {}", encoded.unwrap());
    //}
}
