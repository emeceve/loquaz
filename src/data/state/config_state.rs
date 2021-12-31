use super::contact_state::ContactState;
use druid::{im::Vector, Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct ConfigState {
    pub contacts: Vector<ContactState>,
    pub relays_url: Vector<String>,
}

impl ConfigState {
    pub fn new() -> Self {
        Self {
            contacts: Vector::new(),
            relays_url: Vector::new(),
        }
    }
}
