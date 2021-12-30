use std::collections::HashMap;

use super::contact::Contact;

#[derive(Debug, Clone)]
pub struct Config {
    pub contacts: HashMap<String, Contact>,
    pub relays_url: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            contacts: HashMap::new(),
            relays_url: HashMap::new(),
        }
    }
}
