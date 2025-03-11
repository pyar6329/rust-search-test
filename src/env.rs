use anyhow::{Error, Result, bail};
use envy::Error as EnvyError;
use serde::Deserialize;
use serde::de::Error as _;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_host")]
    pub meilisearch_host: String,
    #[serde(default = "default_token")]
    pub meilisearch_token: String,
}

pub fn default_host() -> String {
    "http://localhost:7700".to_string()
}

pub fn default_token() -> String {
    "masterKey".to_string()
}

impl Config {
    pub fn new() -> Result<Self, Error> {
        let envs = envy::from_env::<Self>().map_err(Error::new)?;
        if envs.meilisearch_host.is_empty() {
            bail!(EnvyError::custom(
                "cannot set env as empty string: MEILISEARCH_HOST"
            ));
        }

        if envs.meilisearch_token.is_empty() {
            bail!(EnvyError::custom(
                "cannot set env as empty string: MEILISEARCH_TOKEN"
            ));
        }

        Ok(envs)
    }
}
