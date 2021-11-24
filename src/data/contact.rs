use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

#[derive(Debug, Data, Clone, Lens, Serialize, Deserialize)]
pub struct Contact {
    pub alias: String,
    pub pk: String,
}

impl Contact {
    pub fn new(alias: &str, pk: &str) -> Self {
        Self {
            alias: alias.into(),
            pk: pk.into(),
        }
    }
}
