use std::str::FromStr;

use druid::{Data, Lens};
use secp256k1::{rand::OsRng, schnorrsig, Secp256k1, SecretKey};

#[derive(Data, Clone, Lens)]
pub struct User {
    pub sk: String,
    pub pk: String,
}

impl User {
    pub fn new(sk: &str, pk: &str) -> Self {
        Self {
            sk: sk.into(),
            pk: pk.into(),
        }
    }

    pub fn generate_keys_from_sk(&mut self) {
        let secp = Secp256k1::new();
        if let Ok(sk) = SecretKey::from_str(&self.sk) {
            let keypair = schnorrsig::KeyPair::from_secret_key(&secp, sk);
            let pk = schnorrsig::PublicKey::from_keypair(&secp, &keypair);
            self.sk = sk.to_string();
            self.pk = pk.to_string();
        } else {
            self.generate_keys();
        }
    }

    pub fn generate_keys(&mut self) {
        let secp = Secp256k1::new();
        let mut rng = OsRng::new().unwrap();
        let (sk, pk) = secp.generate_keypair(&mut rng);
        self.sk = sk.to_string();
        self.pk = pk.to_string()[2..].into();
    }
}
