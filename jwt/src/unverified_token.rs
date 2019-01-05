use crate::crypto;
use crate::error::{Error, CryptoProviderError};
use crate::{UnverifiedJwt, Jwt};

impl <'a> UnverifiedJwt<'a> {
    pub fn verify<'b, Key>(self, key: Key) -> Result<Jwt, Error> 
        where Key: crypto::JwtCryptoProvider
    {
        if let Some(sig) = self.sig {
            let sig = base64::decode_config(sig, base64::URL_SAFE_NO_PAD)?;
            use crate::alg::Algorithm::*;
            match self.alg {
                HS256 => key.verify_hs256(self.header_payload.as_bytes(), &sig),
                HS384 => key.verify_hs384(self.header_payload.as_bytes(), &sig),
                HS512 => key.verify_hs512(self.header_payload.as_bytes(), &sig),
                RS256 => key.verify_rs256(self.header_payload.as_bytes(), &sig),
                RS384 => key.verify_rs384(self.header_payload.as_bytes(), &sig),
                RS512 => key.verify_rs512(self.header_payload.as_bytes(), &sig),
                None => Err(CryptoProviderError::NoAlgorithmSpecified),
            }?;

            Ok(self.skip_verify())
        }
        else {
            Err(Error::SignatureMissing)
        }
    }

    pub fn skip_verify(mut self) -> Jwt {
        self.inner.headers = self.headers;
        self.inner
    }
}