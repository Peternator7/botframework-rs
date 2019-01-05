use crate::{Jwt, JwtBuilder};
use crate::alg::Algorithm;

use serde_json::Value;
use chrono::{Utc, DateTime, TimeZone};

///
impl JwtBuilder {

    pub fn issue(self) -> Jwt {
        let mut jwt = self.jwt;
        jwt.issued_at = Some(Utc::now());
        jwt
    }

    pub fn alg(mut self, alg: Algorithm) -> JwtBuilder {
        self.jwt.alg = alg;
        self
    }

    pub fn issuer(mut self, issuer: &str) -> JwtBuilder {
        self.jwt.issuer = Some(issuer.to_string());
        self
    }

    pub fn subject(mut self, subject: &str) -> JwtBuilder {
        self.jwt.subject = Some(subject.to_string());
        self
    }

    pub fn claim<V>(mut self, key: &str, value: V) -> JwtBuilder 
        where V: std::convert::Into<Value>
    {
        self.jwt.claims.insert(key.to_string(), value.into());
        self
    }

    pub fn expires(mut self, dt: DateTime<Utc>) -> JwtBuilder {
        self.jwt.expires = Some(dt);
        self
    }

    pub fn expires_in(mut self, secs: i64) -> JwtBuilder {
        let exp = Utc::now();
        self.jwt.expires = Some(Utc.timestamp(exp.timestamp() + secs,0));
        self
    }

    pub fn not_before(mut self, dt: DateTime<Utc>) -> JwtBuilder {
        self.jwt.not_before = Some(dt);
        self
    }

    pub fn add_audience(mut self, aud: &str) -> JwtBuilder {
        self.jwt.audience.push(aud.to_string());
        self
    }

    pub fn add_scope(mut self, scope: &str) -> JwtBuilder {
        self.jwt.scopes.push(scope.to_string());
        self
    }

    pub fn jwt_id(mut self, id: &str) -> JwtBuilder {
        self.jwt.jwt_id = Some(id.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::Jwt;

    #[test]
    fn builder_test() {
        let jwt = Jwt::builder()
            .alg(crate::alg::Algorithm::HS256)
            .issuer("my-issuer")
            .subject("my-subject")
            .claim("my-claim","value")
            .claim("second-claim", "value")
            .issue();

        assert_eq!(Some("my-issuer".to_string()), jwt.issuer);        
        assert_eq!(Some("my-subject".to_string()), jwt.subject);    
        assert_eq!("value", jwt.claims["my-claim"]);   
        assert_eq!("value", jwt.claims["second-claim"]);
    }
}