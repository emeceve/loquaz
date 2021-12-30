use druid::{Data, Lens};

use crate::core::entities::contact::Contact;

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

    pub fn from_entity(contact_entity: &Contact) -> ContactState {
        ContactState {
            alias: contact_entity.alias.clone(),
            pk: contact_entity.pk.clone(),
        }
    }
}
