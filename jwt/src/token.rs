use crate::alg::Algorithm;
use crate::crypto;
use crate::error::{Error, CryptoProviderError};
use crate::{Jwt, JwtBuilder, JwtHeaders, UnverifiedJwt};

use std::str;
use std::collections::HashMap;

impl Jwt {
    /// Create a new instance of `Jwt`. This is just a synonym for `Default::default()`. You may also
    /// want to consider using `.builder()` to construct new `Jwt` tokens.
    pub fn new() -> Jwt {
        Self::default()
    }

    /// Construct a new instance of JwtBuilder. The builder pattern can be used for constructing new tokens
    /// ```rust
    /// use jwt
    pub fn builder() -> JwtBuilder {
        JwtBuilder {
            jwt: Jwt::new(),
        }
    }

    pub fn decode<'a>(input: &'a str) -> Result<UnverifiedJwt<'a>, Error> {
        let mut token;
        let mut alg = Algorithm::None;
        let mut headers;

        let mut splits = input.split('.');
        if let Some(header) = splits.next() {
            let header = base64::decode(header)?;
            let header = str::from_utf8(&header)?;
            headers = serde_json::from_str::<HashMap<String,String>>(header)?;

            // Dumb keep alive variable
            let t = headers.remove("typ");
            match t.as_ref().map(|h| h.as_ref()) {
                Some("JWT") => {},
                _ => return Err(Error::MalformedJwt), // Wrong token type. Fail
            }

            if let Some(a) = headers.remove("alg") {
                use std::str::FromStr;
                alg = Algorithm::from_str(&a)?;
            }
        } else {
            // No headers. Needs to fail
            return Err(Error::MalformedJwt);
        }

        if let Some(payload) = splits.next() {
            let payload = base64::decode(payload)?;
            let payload = str::from_utf8(&payload)?;
            token = serde_json::from_str::<Jwt>(payload)?;
            token.alg = alg;
            // token.headers = headers;
        } else {
            // No payload, needs to fail
            return Err(Error::MalformedJwt);
        }

        // A signature isn't required.
        if let Some(sig) = splits.next() {
            // The payload is signed so we hold onto the signature and the encrypted message
            // this way we can verify it later.
            // This also let's us inspect the message before we attempt to verify it.
            let last_period = input.rfind('.').unwrap();
            Ok(UnverifiedJwt {
                header_payload: &input[0..last_period],
                sig: Some(sig),
                alg: alg,
                headers: headers,
                inner: token,
            })
        }
        else {
            Ok(UnverifiedJwt {
                header_payload: input,
                sig: None,
                alg: alg,
                headers: headers,
                inner: token,
            })
        }
    }

    pub fn encode<'a, Key>(&self, key: Key) -> Result<String, Error>
        where Key: crypto::JwtCryptoProvider
    {
        let mut output = String::new();
        let payload = serde_json::to_string(self)?;
        let header = serde_json::to_string(&JwtHeaders {
            headers: &self.headers,
            alg: self.alg,
        })?;

        // Write the body of jwt.
        base64::encode_config_buf(header.as_bytes(), base64::URL_SAFE_NO_PAD, &mut output);
        output.push('.');
        base64::encode_config_buf(payload.as_bytes(), base64::URL_SAFE_NO_PAD, &mut output);

        if let Algorithm::None = self.alg {
            return Ok(output);
        } else {
            use crate::alg::Algorithm::*;
            let sig = match self.alg {
                    HS256 => key.sign_hs256(output.as_bytes()),
                    HS384 => key.sign_hs384(output.as_bytes()),
                    HS512 => key.sign_hs512(output.as_bytes()),
                    RS256 => key.sign_rs256(output.as_bytes()),
                    RS384 => key.sign_rs384(output.as_bytes()),
                    RS512 => key.sign_rs512(output.as_bytes()),
                    _ => Err(CryptoProviderError::UnimplementedAlgorithm),
                }?;

            let sig = base64::encode_config(&sig, base64::URL_SAFE_NO_PAD);
            output.push('.');
            output.push_str(&sig);
            Ok(output)
        }
    }
}

impl std::default::Default for Jwt {
    fn default() -> Self {
        Jwt {
            issuer: None,
            subject: None,
            audience: Vec::new(),
            not_before: None,
            expires: None,
            issued_at: None,
            jwt_id: None,
            alg: Algorithm::None,
            claims: HashMap::new(),
            headers: HashMap::new(),
            scopes: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    // fn encode
}