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

    pub fn issuer<Str>(mut self, issuer: Str) -> JwtBuilder
        where Str: Into<String>
    {
        self.jwt.issuer = Some(issuer.into());
        self
    }

    pub fn subject<Str>(mut self, subject: Str) -> JwtBuilder
        where Str: Into<String>
    {
        self.jwt.subject = Some(subject.into());
        self
    }

    pub fn claim<Str, V>(mut self, key: Str, value: V) -> JwtBuilder
        where Str: Into<String>,
              V: std::convert::Into<Value>
    {
        self.jwt.claims.insert(key.into(), value.into());
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

    /// Add an allowed audience to the token. This function can be called multiple times
    /// and the audience values will be appended instead of replace.
    pub fn audience<Str>(mut self, aud: Str) -> JwtBuilder
        where Str: Into<String>
    {
        self.jwt.audience.push(aud.into());
        self
    }

    /// Add a scope to the list of valid JWT scopes for the token. This function is intended
    /// to be called multiple times
    pub fn scope<Str>(mut self, scope: Str) -> JwtBuilder
        where Str: Into<String>
    {
        self.jwt.scopes.push(scope.into());
        self
    }

    pub fn jwt_id<Str>(mut self, id: Str) -> JwtBuilder
        where Str: Into<String>
    {
        self.jwt.jwt_id = Some(id.into());
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