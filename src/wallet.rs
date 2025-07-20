use ed25519_dalek::{SigningKey, VerifyingKey, Signer};
use rand::rngs::OsRng;
use sha3::{Digest, Sha3_256};
use hex;

pub struct Wallet {
    pub signing_key: SigningKey,
}

impl Wallet {
    pub fn get_address(&self) -> String {
        let public_key: VerifyingKey = self.signing_key.verifying_key();
        let hash = Sha3_256::digest(public_key.as_bytes());
        format!("0x{}", hex::encode(&hash))
    }

    #[allow(dead_code)]
    pub fn sign(&self, msg: &[u8]) -> Vec<u8> {
        self.signing_key.sign(msg).to_bytes().to_vec()
    }
}

pub fn generate_wallet() -> Wallet {
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    Wallet { signing_key }
}
