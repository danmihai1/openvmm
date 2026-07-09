// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! ECDSA implementation using SymCrypt.

use super::EcdsaCurve;
use super::EcdsaError;

fn err(e: symcrypt::errors::SymCryptError, op: &'static str) -> EcdsaError {
    EcdsaError(crate::BackendError::SymCrypt(e, op))
}

#[repr(transparent)] // Needed for the transmute in as_pub.
pub struct EcdsaKeyPairInner {
    key: symcrypt::ecc::EcKey,
}

impl EcdsaKeyPairInner {
    pub fn generate(curve: EcdsaCurve) -> Result<Self, EcdsaError> {
        let curve_type = match curve {
            EcdsaCurve::P384 => symcrypt::ecc::CurveType::NistP384,
        };
        let key =
            symcrypt::ecc::EcKey::generate_key_pair(curve_type, symcrypt::ecc::EcKeyUsage::EcDsa)
                .map_err(|e| err(e, "generating ECDSA key pair"))?;
        Ok(Self { key })
    }

    pub fn sign_prehash(&self, hash: &[u8]) -> Result<Vec<u8>, EcdsaError> {
        self.key.ecdsa_sign(hash).map_err(|e| err(e, "ECDSA sign"))
    }

    pub(crate) fn as_pub(&self) -> &EcdsaPublicKeyInner {
        // SAFETY: EcdsaPublicKeyInner is just a wrapper around the same EcKey.
        unsafe { std::mem::transmute::<&EcdsaKeyPairInner, &EcdsaPublicKeyInner>(self) }
    }
}

#[repr(transparent)] // Needed for the transmute in as_pub.
pub struct EcdsaPublicKeyInner {
    key: symcrypt::ecc::EcKey,
}

impl EcdsaPublicKeyInner {
    pub fn new(curve: EcdsaCurve, public_key: &[u8]) -> Result<Self, EcdsaError> {
        // Enforce the exact `Qx || Qy` length so all backends reject
        // non-canonical encodings identically (the OpenSSL backend does the
        // same before constructing its point).
        if public_key.len() != curve.key_size() * 2 {
            return Err(err(
                symcrypt::errors::SymCryptError::InvalidArgument,
                "validating ECDSA public key length",
            ));
        }
        let curve_type = match curve {
            EcdsaCurve::P384 => symcrypt::ecc::CurveType::NistP384,
        };
        let key = symcrypt::ecc::EcKey::set_public_key(
            curve_type,
            public_key,
            symcrypt::ecc::EcKeyUsage::EcDsa,
        )
        .map_err(|e| err(e, "importing public key"))?;
        Ok(Self { key })
    }

    pub fn verify_prehash(&self, hash: &[u8], signature: &[u8]) -> Result<bool, EcdsaError> {
        match self.key.ecdsa_verify(signature, hash) {
            Ok(()) => Ok(true),
            // `SignatureVerificationFailure` is the expected error for a
            // signature that does not match. `InvalidArgument` occurs when the
            // signature is malformed (e.g. wrong length, or a component that is
            // out of range), which likewise means "does not verify".
            Err(
                symcrypt::errors::SymCryptError::SignatureVerificationFailure
                | symcrypt::errors::SymCryptError::InvalidArgument,
            ) => Ok(false),
            Err(e) => Err(err(e, "ECDSA verify")),
        }
    }

    pub fn public_key_bytes(&self) -> Result<Vec<u8>, EcdsaError> {
        self.key
            .export_public_key()
            .map_err(|e| err(e, "exporting public key"))
    }
}
