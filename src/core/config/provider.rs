use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufReader, Error},
    path::PathBuf,
};

use crate::core::entities::{config::Config, contact::Contact};

const NOSTR_DIR_NAME: &str = r".nostr_chat";
const CONFIG_FILENAME: &str = "config.json";

#[derive(Clone)]
pub struct ConfigProvider {
    state: Config,
}

impl ConfigProvider {
    fn new() -> Self {
        Self {
            state: Config::new(),
        }
    }

    pub fn get(&self) -> Config {
        self.state.clone()
    }

    pub fn add_contact(&mut self, new_contact: Contact) -> Result<(), Error> {
        self.state
            .contacts
            .insert(new_contact.pk.clone(), new_contact);
        self.save()
    }
    pub fn remove_contact(&mut self, contact: Contact) -> Result<(), Error> {
        self.state.contacts.remove(&contact.pk);
        self.save()
    }

    pub fn add_relay(&mut self, new_relay_url: String) -> Result<(), Error> {
        self.state
            .relays_url
            .insert(new_relay_url.clone(), new_relay_url.clone());
        self.save()
    }
    pub fn remove_relay(&mut self, relay_url: &str) -> Result<(), Error> {
        self.state.relays_url.remove(relay_url);
        self.save()
    }

    pub fn delete_contact(&mut self, pk: &str) {
        self.state.contacts.remove(pk);
        self.save().unwrap();
    }

    pub fn save(&self) -> Result<(), Error> {
        let contacts: Vec<Contact> = self
            .state
            .contacts
            .iter()
            .map(|(k, v)| v.to_owned())
            .collect();
        let relays_url: Vec<String> = self
            .state
            .relays_url
            .iter()
            .map(|(k, v)| v.to_owned())
            .collect();
        let config_file = ConfigFile::new(contacts, relays_url);
        let serialized = serde_json::to_string_pretty(&config_file)?;
        let config_path = Self::get_path();

        std::fs::write(Self::get_config_path(), serialized)?;
        Ok(())
    }

    pub fn load() -> Self {
        let mut dir = Self::get_path();
        fs::create_dir_all(&dir);
        let file = File::open(&Self::get_config_path());

        match file {
            Ok(file) => {
                let reader = BufReader::new(file);
                let config_file: ConfigFile =
                    serde_json::from_reader(reader).unwrap_or(ConfigFile::new(vec![], vec![]));
                let mut contacts = HashMap::new();
                let mut relays_url = HashMap::new();

                config_file.contacts.into_iter().for_each(|v| {
                    contacts.insert(v.pk.clone(), v);
                });

                config_file.relays_url.into_iter().for_each(|v| {
                    relays_url.insert(v.clone(), v);
                });

                let mut config = Config::new();
                config.contacts = contacts;
                config.relays_url = relays_url;
                Self { state: config }
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
                println!(
                    "Using the following directory: {}",
                    nostr_dir_path.display(),
                );
                nostr_dir_path
            }
            None => {
                println!("Impossible to get your home dir");
                let mut local_dir = PathBuf::new();
                local_dir.push(NOSTR_DIR_NAME);
                local_dir
            }
        };
        dir
    }
}

#[derive(Serialize, Deserialize)]
struct ConfigFile {
    pub relays_url: Vec<String>,
    pub contacts: Vec<Contact>,
}

impl ConfigFile {
    pub fn new(contacts: Vec<Contact>, relays_url: Vec<String>) -> Self {
        Self {
            contacts,
            relays_url,
        }
    }
}
