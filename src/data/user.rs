use std::sync::Arc;

use druid::{Data, Lens};

#[derive(Data, Clone, Lens)]
pub struct User {
    //TODO use typed keys structures
    pub sk: String,
    pub pk: String,
    pub keys: Option<Arc<nostr::Keys>>,
}

impl User {
    pub fn new(sk: &str, pk: &str) -> Self {
        Self {
            sk: sk.into(),
            pk: pk.into(),
            keys: None,
        }
    }

    pub fn generate_keys_from_sk(&mut self) {
        if let Ok(keys) = nostr::Keys::new(&self.sk) {
            self.keys = Some(Arc::new(keys.clone()));
            self.pk = keys.public_key_as_str();
        } else {
            self.generate_keys();
        }
    }

    pub fn generate_keys(&mut self) {
        // TODO fix this in nostr-rs, should panic if os random fails
        let keys = nostr::Keys::generate_from_os_random().unwrap();
        self.keys = Some(Arc::new(keys.clone()));

        self.pk = keys.public_key_as_str();
        self.sk = keys.secret_key_as_str().expect("'Should return SK");
    }
}
