use secp256k1::schnorrsig::PublicKey;

pub struct User {
    pub keys: nostr::Keys,
}

impl User {
    pub fn new() -> Self {
        Self {
            keys: nostr::Keys::generate_from_os_random().unwrap(),
        }
    }

    pub fn import_sk(&mut self, sk: &str) {
        if let Ok(keys) = nostr::Keys::new(sk) {
            self.keys = keys;
        }
    }

    pub fn get_keys(&self) -> nostr::Keys {
        self.keys.clone()
    }

    pub fn get_pk(&self) -> PublicKey {
        self.keys.public_key.clone()
    }
}
