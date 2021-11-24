use druid::{
    im::{vector, Vector},
    Data, Lens,
};
use serde::{Deserialize, Serialize};

use crate::data::contact::Contact;
use std::{
    fs::{self, File},
    io::{BufReader, Error},
    path::PathBuf,
};

const NOSTR_DIR_NAME: &str = r".nostr_chat";
const CONFIG_FILENAME: &str = "config.json";

#[derive(Data, Clone, Lens)]
pub struct Config {
    pub contacts: Vector<Contact>,
    pub ws_url: String,
}

impl Config {
    fn new() -> Self {
        Self {
            contacts: vector![],
            ws_url: "".into(),
        }
    }

    pub fn add_contact(&mut self, new_contact: &Contact) {
        self.contacts.push_back(new_contact.clone());
        self.save().unwrap();
    }

    pub fn delete_contact(&mut self, pk: &str) {
        self.contacts.retain(|contact| contact.pk != pk);
        self.save().unwrap();
    }

    pub fn save(&self) -> Result<(), Error> {
        let contacts_vec: Vec<Contact> = self.contacts.iter().map(|c| c.to_owned()).collect();
        let config_file = ConfigFile::new(&contacts_vec, &self.ws_url);
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
                let config: ConfigFile =
                    serde_json::from_reader(reader).unwrap_or(ConfigFile::new(&vec![], ""));
                Self {
                    contacts: Vector::from(config.contacts),
                    ws_url: config.ws_url,
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
    pub ws_url: String,
    pub contacts: Vec<Contact>,
}

impl ConfigFile {
    pub fn new(contacts: &Vec<Contact>, ws_url: &str) -> Self {
        Self {
            contacts: contacts.clone(),
            ws_url: ws_url.into(),
        }
    }
}
