use crate::error::CryptoProviderError;

mod rsa_providers;
mod hmac_providers;

#[allow(unused_variables)] 
pub trait JwtCryptoProvider {
    fn sign_hs256(&self, msg: &[u8]) -> Result<Vec<u8>, CryptoProviderError> {
        Err(CryptoProviderError::InvalidKey)
    }

    fn sign_hs384(&self, msg: &[u8]) -> Result<Vec<u8>, CryptoProviderError> {
        Err(CryptoProviderError::InvalidKey)
    }

    fn sign_hs512(&self, msg: &[u8]) -> Result<Vec<u8>, CryptoProviderError> {
        Err(CryptoProviderError::InvalidKey)
    }

    fn verify_hs256(&self, msg: &[u8], sig: &[u8]) -> Result<(), CryptoProviderError> {
        Err(CryptoProviderError::InvalidKey)
    }

    fn verify_hs384(&self, msg: &[u8], sig: &[u8]) -> Result<(), CryptoProviderError> {
        Err(CryptoProviderError::InvalidKey)
    }

    fn verify_hs512(&self, msg: &[u8], sig: &[u8]) -> Result<(), CryptoProviderError> {
        Err(CryptoProviderError::InvalidKey)
    }

    fn sign_rs256(&self, msg: &[u8]) -> Result<Vec<u8>, CryptoProviderError> {
        Err(CryptoProviderError::InvalidKey)
    }

    fn sign_rs384(&self, msg: &[u8]) -> Result<Vec<u8>, CryptoProviderError> {
        Err(CryptoProviderError::InvalidKey)
    }

    fn sign_rs512(&self, msg: &[u8]) -> Result<Vec<u8>, CryptoProviderError> {
        Err(CryptoProviderError::InvalidKey)
    }

    fn verify_rs256(&self, msg: &[u8], sig: &[u8]) -> Result<(), CryptoProviderError> {
        Err(CryptoProviderError::InvalidKey)
    }

    fn verify_rs384(&self, msg: &[u8], sig: &[u8]) -> Result<(), CryptoProviderError> {
        Err(CryptoProviderError::InvalidKey)
    }

    fn verify_rs512(&self, msg: &[u8], sig: &[u8]) -> Result<(), CryptoProviderError> {
        Err(CryptoProviderError::InvalidKey)
    }
}

impl <'a, T> JwtCryptoProvider for &'a [T]
    where T: JwtCryptoProvider 
{
    fn sign_hs256(&self, msg: &[u8]) -> Result<Vec<u8>, CryptoProviderError> {
        for key in self.iter() {
            return key.sign_hs256(msg);
        }

        Err(CryptoProviderError::InvalidKey)
    }

    fn sign_hs384(&self, msg: &[u8]) -> Result<Vec<u8>, CryptoProviderError> {
        for key in self.iter() {
            return key.sign_hs384(msg);
        }

        Err(CryptoProviderError::InvalidKey)
    }

    fn sign_hs512(&self, msg: &[u8]) -> Result<Vec<u8>, CryptoProviderError> {
        for key in self.iter() {
            return key.sign_hs512(msg);
        }

        Err(CryptoProviderError::InvalidKey)
    }

    fn verify_hs256(&self, msg: &[u8], sig: &[u8]) -> Result<(), CryptoProviderError> {
        for key in self.iter() {
            if let Ok(()) = key.verify_hs256(msg, sig) {
                return Ok(());
            }
        }

        Err(CryptoProviderError::InvalidKey)
    }

    fn verify_hs384(&self, msg: &[u8], sig: &[u8]) -> Result<(), CryptoProviderError> {
        for key in self.iter() {
            if let Ok(()) = key.verify_hs384(msg, sig) {
                return Ok(());
            }
        }

        Err(CryptoProviderError::InvalidKey)
    }

    fn verify_hs512(&self, msg: &[u8], sig: &[u8]) -> Result<(), CryptoProviderError> {
        for key in self.iter() {
            if let Ok(()) = key.verify_hs512(msg, sig) {
                return Ok(());
            }
        }

        Err(CryptoProviderError::InvalidKey)
    }

    fn sign_rs256(&self, msg: &[u8]) -> Result<Vec<u8>, CryptoProviderError> {
        for key in self.iter() {
            return key.sign_rs256(msg);
        }

        Err(CryptoProviderError::InvalidKey)
    }

    fn sign_rs384(&self, msg: &[u8]) -> Result<Vec<u8>, CryptoProviderError> {
        for key in self.iter() {
            return key.sign_rs384(msg);
        }

        Err(CryptoProviderError::InvalidKey)
    }

    fn sign_rs512(&self, msg: &[u8]) -> Result<Vec<u8>, CryptoProviderError> {
        for key in self.iter() {
            return key.sign_rs512(msg);
        }

        Err(CryptoProviderError::InvalidKey)
    }

    fn verify_rs256(&self, msg: &[u8], sig: &[u8]) -> Result<(), CryptoProviderError> {
        for key in self.iter() {
            if let Ok(()) = key.verify_rs256(msg, sig) {
                return Ok(());
            }
        }

        Err(CryptoProviderError::InvalidKey)
    }

    fn verify_rs384(&self, msg: &[u8], sig: &[u8]) -> Result<(), CryptoProviderError> {
        for key in self.iter() {
            if let Ok(()) = key.verify_rs384(msg, sig) {
                return Ok(());
            }
        }

        Err(CryptoProviderError::InvalidKey)
    }

    fn verify_rs512(&self, msg: &[u8], sig: &[u8]) -> Result<(), CryptoProviderError> {
        for key in self.iter() {
            if let Ok(()) = key.verify_rs512(msg, sig) {
                return Ok(());
            }
        }

        Err(CryptoProviderError::InvalidKey)
    }
}
