use log::{info, warn};
use secp256k1::schnorrsig::PublicKey;
use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufReader, Error},
    path::PathBuf,
};

const NOSTR_DIR_NAME: &str = r".nostr_chat";
const CONFIG_FILENAME: &str = "config.json";

#[derive(Clone)]
pub struct ConfigProvider {
    contacts: HashMap<String, Contact>,
    relays_url: HashMap<String, String>,
}

impl ConfigProvider {
    fn new() -> Self {
        Self {
            contacts: HashMap::new(),
            relays_url: HashMap::new(),
        }
    }

    pub fn add_contact(&mut self, new_contact: Contact) -> Result<(), Error> {
        self.contacts
            .insert(new_contact.pk.to_string(), new_contact);
        self.save()
    }
    pub fn remove_contact(&mut self, contact: Contact) -> Result<(), Error> {
        self.contacts.remove(&contact.pk.to_string());
        self.save()
    }

    pub fn add_relay(&mut self, new_relay_url: String) -> Result<(), Error> {
        self.relays_url
            .insert(new_relay_url.clone(), new_relay_url.clone());
        self.save()
    }
    pub fn remove_relay(&mut self, relay_url: &str) -> Result<(), Error> {
        self.relays_url.remove(relay_url);
        self.save()
    }

    pub fn delete_contact(&mut self, pk: &str) {
        self.contacts.remove(pk);
        self.save().unwrap();
    }

    pub fn list_contacts(&self) -> Vec<Contact> {
        self.contacts.iter().map(|(k, v)| v.to_owned()).collect()
    }

    pub fn list_relays_url(&self) -> Vec<String> {
        self.relays_url.iter().map(|(k, v)| v.to_owned()).collect()
    }

    pub fn save(&self) -> Result<(), Error> {
        let contacts: Vec<Contact> = self.list_contacts();
        let relays_url: Vec<String> = self.list_relays_url();
        let config_file = Config::new(contacts, relays_url);
        let serialized = serde_json::to_string_pretty(&config_file)?;
        let config_path = Self::get_path();

        std::fs::write(Self::get_config_path(), serialized)?;
        Ok(())
    }

    pub fn load() -> Self {
        let mut dir = Self::get_path();
        info!("Loading configs from file {}", dir.display());
        fs::create_dir_all(&dir);
        let file = File::open(&Self::get_config_path());

        match file {
            Ok(file) => {
                let reader = BufReader::new(file);
                let config: Config =
                    serde_json::from_reader(reader).unwrap_or(Config::new(vec![], vec![]));
                let mut contacts = HashMap::new();
                let mut relays_url = HashMap::new();

                config.contacts.into_iter().for_each(|v| {
                    contacts.insert(v.pk.to_string(), v);
                });

                config.relays_url.into_iter().for_each(|v| {
                    relays_url.insert(v.clone(), v);
                });

                Self {
                    contacts,
                    relays_url,
                }
            }
            Err(_) => Self::new(),
        }
    }

    fn get_config_path() -> PathBuf {
        let mut path = Self::get_path();
        path.push(CONFIG_FILENAME);
        path
    }
    fn get_path() -> PathBuf {
        let dir = match home::home_dir() {
            Some(path) => {
                let mut nostr_dir_path = PathBuf::new();
                nostr_dir_path.push(path);
                nostr_dir_path.push(NOSTR_DIR_NAME);
                nostr_dir_path
            }
            None => {
                warn!("Impossible to get your home dir");
                let mut local_dir = PathBuf::new();
                local_dir.push(NOSTR_DIR_NAME);
                local_dir
            }
        };
        dir
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub relays_url: Vec<String>,
    pub contacts: Vec<Contact>,
}

impl Config {
    pub fn new(contacts: Vec<Contact>, relays_url: Vec<String>) -> Self {
        Self {
            contacts,
            relays_url,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Contact {
    pub alias: String,
    pub pk: PublicKey,
}

impl Contact {
    pub fn new(alias: &str, pk: PublicKey) -> Self {
        Self {
            alias: alias.into(),
            pk,
        }
    }
}
