use druid::{Data, Lens};

#[derive(Data, Clone, Lens)]
pub struct UserState {
    pub sk: String,
    pub pk: String,
}

impl UserState {
    pub fn new(sk: &str, pk: &str) -> Self {
        Self {
            sk: sk.into(),
            pk: pk.into(),
        }
    }
}
