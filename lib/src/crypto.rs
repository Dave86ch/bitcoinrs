use crate::sha256::Hash;
use crate::util::Saveable;
use k256::ecdsa::signature::Verifier;
use k256::ecdsa::{signature::Signer, Signature as ECDSASignature, SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};
use spki::EncodePublicKey;
use std::io::{Error as IoError, ErrorKind as IoErrorKind, Read, Result as IoResult, Write};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Signature(ECDSASignature);
impl Signature {
    // sign a crate::types::TransactionOutput from its Sha256 hash
    pub fn sign_output(output_hash: &Hash, private_key: &PrivateKey) -> Self {
        let signing_key = &private_key.0;
        let signature = signing_key.sign(&output_hash.as_bytes());
        Signature(signature)
    }
    pub fn verify(&self, output_hash: &Hash, public_key: &PublicKey) -> bool {
        public_key
            .0
            .verify(&output_hash.as_bytes(), &self.0)
            .is_ok()
    }
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct PublicKey(VerifyingKey);
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PrivateKey(#[serde(with = "signkey_serde")] pub SigningKey);

mod signkey_serde {
    use serde::Deserialize;
    pub fn serialize<S>(key: &super::SigningKey, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&key.to_bytes())
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<super::SigningKey, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: Vec<u8> = Vec::<u8>::deserialize(deserializer)?;
        Ok(super::SigningKey::from_slice(&bytes).unwrap())
    }
}
impl PrivateKey {
    pub fn new_key() -> Self {
        PrivateKey(SigningKey::random(&mut rand::thread_rng()))
    }
    pub fn public_key(&self) -> PublicKey {
        PublicKey(self.0.verifying_key().clone())
    }
}
impl Saveable for PrivateKey {
    fn load<I: Read>(reader: I) -> IoResult<Self> {
        ciborium::de::from_reader(reader)
            .map_err(|_| IoError::new(IoErrorKind::InvalidData, "Failed to deserialize PrivateKey"))
    }

    fn save<O: Write>(&self, writer: O) -> IoResult<()> {
        ciborium::ser::into_writer(self, writer).map_err(|_| {
            IoError::new(IoErrorKind::InvalidData, "Failed to serialize PrivateKey")
        })?;
        Ok(())
    }
}

// save and load as PEM
impl Saveable for PublicKey {
    fn load<I: Read>(mut reader: I) -> IoResult<Self> {
        // read PEM-encoded public key into string
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;

        // decode the public key from PEM
        let public_key = buf
            .parse()
            .map_err(|_| IoError::new(IoErrorKind::InvalidData, "Failed to parse PublicKey"))?;

        Ok(PublicKey(public_key))
    }

    fn save<O: Write>(&self, mut writer: O) -> IoResult<()> {
        let s = self
            .0
            .to_public_key_pem(Default::default())
            .map_err(|_| IoError::new(IoErrorKind::InvalidData, "Failed to serialize PublicKey"))?;

        writer.write_all(s.as_bytes())?;
        Ok(())
    }
}
