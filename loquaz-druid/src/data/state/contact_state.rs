use druid::{Data, Lens};

#[derive(Debug, Data, Clone, Lens)]
pub struct ContactState {
    pub alias: String,
    pub pk: String,
}

impl ContactState {
    pub fn new(alias: &str, pk: &str) -> Self {
        Self {
            alias: alias.into(),
            pk: pk.into(),
        }
    }
}
