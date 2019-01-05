use crate::error::CryptoProviderError;
use crate::crypto::JwtCryptoProvider;

use ring::signature::primitive::verify_rsa;
use ring::signature;
use untrusted;

fn verify_rs_internal(alg: &signature::RSAParameters, 
    ne: &AssymetricKey, 
    msg: &[u8], 
    sig: &[u8]) -> Result<(), CryptoProviderError> 
{
    let n = untrusted::Input::from(ne.n);
    let e = untrusted::Input::from(ne.e);
    
    verify_rsa(alg,
        (n,e),
        untrusted::Input::from(msg),
        untrusted::Input::from(sig),
    ).map_err(|_| CryptoProviderError::InvalidKey)
}

pub struct AssymetricKey<'a> {
    pub n: &'a [u8],
    pub e: &'a [u8],
}

impl <'a> From<&'a (&'a [u8], &'a [u8])> for AssymetricKey<'a> {
    fn from (tup: &'a (&'a [u8], &'a [u8])) -> AssymetricKey<'a>{
        let (n, e) = tup;
        AssymetricKey { n, e }
    }
}

impl <'a> AssymetricKey<'a> {
    #[allow(dead_code)]
    pub fn new<'b>(n: &'b [u8], e: &'b [u8]) -> AssymetricKey<'b> {
        AssymetricKey { n, e }
    }
}

impl <'a> JwtCryptoProvider for AssymetricKey<'a> {

    fn verify_rs256(&self, msg: &[u8], sig: &[u8]) -> Result<(), CryptoProviderError> {
        let alg = &signature::RSA_PKCS1_2048_8192_SHA256;
        verify_rs_internal(alg, self,  msg, sig)
    }

    fn verify_rs384(&self, msg: &[u8], sig: &[u8]) -> Result<(), CryptoProviderError> {
        let alg = &signature::RSA_PKCS1_2048_8192_SHA384;
        verify_rs_internal(alg, self,  msg, sig)
    }

    fn verify_rs512(&self, msg: &[u8], sig: &[u8]) -> Result<(), CryptoProviderError> {
        let alg = &signature::RSA_PKCS1_2048_8192_SHA512;
        verify_rs_internal(alg, self,  msg, sig)
    }
}

pub struct DerEncodedPublicKey<'a>(pub &'a [u8]);

impl <'a> From<&'a [u8]> for DerEncodedPublicKey<'a> {
    fn from (bytes: &'a [u8]) -> DerEncodedPublicKey<'a>{
        DerEncodedPublicKey(bytes)
    }
}

impl <'a> JwtCryptoProvider for DerEncodedPublicKey<'a> {

    fn verify_rs256(&self, msg: &[u8], sig: &[u8]) -> Result<(), CryptoProviderError> {
        let alg = &signature::RSA_PKCS1_2048_8192_SHA256;
        ring::signature::verify(alg, 
                untrusted::Input::from(self.0), 
                untrusted::Input::from(msg), 
                untrusted::Input::from(sig)
            ).map_err(|_| CryptoProviderError::InvalidKey)
    }

    fn verify_rs384(&self, msg: &[u8], sig: &[u8]) -> Result<(), CryptoProviderError> {
        let alg = &signature::RSA_PKCS1_2048_8192_SHA384;
        ring::signature::verify(alg, 
                untrusted::Input::from(self.0), 
                untrusted::Input::from(msg), 
                untrusted::Input::from(sig)
            ).map_err(|_| CryptoProviderError::InvalidKey)
    }

    fn verify_rs512(&self, msg: &[u8], sig: &[u8]) -> Result<(), CryptoProviderError> {
        let alg = &signature::RSA_PKCS1_2048_8192_SHA512;
        signature::verify(alg, 
                untrusted::Input::from(self.0), 
                untrusted::Input::from(msg), 
                untrusted::Input::from(sig)
            ).map_err(|_| CryptoProviderError::InvalidKey)
    }
}