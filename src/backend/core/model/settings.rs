#![cfg(feature = "ssr")]

use config::ConfigError;
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub sounds: SoundSettings,
    pub accounts: AccountsSettings,
}

#[derive(Deserialize, Debug)]
pub struct SoundSettings {
    pub articles: HashMap<String, Vec<String>>,
    pub generic: Vec<String>,
    pub failed: Vec<String>,
    pub withdraw: Vec<String>,
    pub deposit: Vec<String>,
}

impl SoundSettings {
    pub fn get_all_sounds(&self) -> Vec<String> {
        let mut all_sounds: Vec<String> = Vec::new();

        for (_, value) in self.articles.iter() {
            all_sounds.append(&mut value.clone());
        }

        all_sounds.append(&mut self.generic.clone());
        all_sounds.append(&mut self.failed.clone());
        all_sounds.append(&mut self.withdraw.clone());
        all_sounds.append(&mut self.deposit.clone());

        all_sounds
    }
}

#[derive(Deserialize, Debug)]
pub struct AccountsSettings {
    pub upper_limit: i64,
    pub lower_limit: i64,
}

impl Settings {
    pub fn new(config_path: PathBuf) -> Result<Self, ConfigError> {
        use config::Config;

        let builder = Config::builder()
            .add_source(config::File::with_name(
                config_path.to_str().unwrap().rsplit_once(".").unwrap().0,
            ))
            .add_source(config::Environment::with_prefix("STRICHLISTE"));

        builder.build()?.try_deserialize()
    }
}
