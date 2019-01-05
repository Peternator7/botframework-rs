use crate::error::CryptoProviderError;
use crate::crypto::JwtCryptoProvider;

use ring::{digest, hmac};
use std::iter::Iterator;

fn sign_hs_internal(alg: hmac::SigningKey, msg: &[u8]) -> Result<Vec<u8>, CryptoProviderError> {
    let sig = hmac::sign(&alg, msg);
    Ok(sig.as_ref().iter().cloned().collect())
}

fn verify_hs_internal(alg: hmac::VerificationKey, msg: &[u8], sig: &[u8]) -> Result<(), CryptoProviderError> {
    hmac::verify(&alg, msg, sig).map_err(|_| CryptoProviderError::ValidationError)
}

impl <'a> JwtCryptoProvider for &'a [u8] {
    fn sign_hs256(&self, msg: &[u8]) -> Result<Vec<u8>, CryptoProviderError> {
        sign_hs_internal(hmac::SigningKey::new(&digest::SHA256, self), msg)
    }

    fn sign_hs384(&self, msg: &[u8]) -> Result<Vec<u8>, CryptoProviderError> {
        sign_hs_internal(hmac::SigningKey::new(&digest::SHA384, self), msg)
    }

    fn sign_hs512(&self, msg: &[u8]) -> Result<Vec<u8>, CryptoProviderError> {
        sign_hs_internal(hmac::SigningKey::new(&digest::SHA512, self), msg)
    }

    fn verify_hs256(&self, msg: &[u8], sig: &[u8]) -> Result<(), CryptoProviderError> {
        verify_hs_internal(hmac::VerificationKey::new(&digest::SHA256, self), msg, sig)
    }

    fn verify_hs384(&self, msg: &[u8], sig: &[u8]) -> Result<(), CryptoProviderError> {
        verify_hs_internal(hmac::VerificationKey::new(&digest::SHA384, self), msg, sig)
    }

    fn verify_hs512(&self, msg: &[u8], sig: &[u8]) -> Result<(), CryptoProviderError> {
        verify_hs_internal(hmac::VerificationKey::new(&digest::SHA512, self), msg, sig)
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::JwtCryptoProvider;
    use crate::error::CryptoProviderError;

    #[test]
    fn simple_sign() {
        let secret = "secret".as_bytes();
        let message = "message".as_bytes();

        secret.sign_hs256(message).unwrap();
        secret.sign_hs384(message).unwrap();
        secret.sign_hs512(message).unwrap();
    }

    #[test]
    fn sign_and_verify_hs256() {
        let secret = "secret".as_bytes();
        let message = "message".as_bytes();

        let signed = secret.sign_hs256(message).unwrap();
        secret.verify_hs256(message, &*signed).unwrap();
    }

    #[test]
    fn sign_and_verify_hs384() {
        let secret = "secret".as_bytes();
        let message = "message".as_bytes();

        let signed = secret.sign_hs384(message).unwrap();
        secret.verify_hs384(message, &*signed).unwrap();
    }

    #[test]
    fn sign_and_verify_hs512() {
        let secret = "secret".as_bytes();
        let message = "message".as_bytes();

        let signed = secret.sign_hs512(message).unwrap();
        secret.verify_hs512(message, &*signed).unwrap();
    }

    #[test]
    fn verify_error () {
        let secret = "secret".as_bytes();
        let message = "message".as_bytes();
        let sig = "incorrect".as_bytes();
        assert_eq!(Err(CryptoProviderError::ValidationError), secret.verify_hs256(message, sig));
        assert_eq!(Err(CryptoProviderError::ValidationError), secret.verify_hs384(message, sig));
        assert_eq!(Err(CryptoProviderError::ValidationError), secret.verify_hs512(message, sig));
    }
}
