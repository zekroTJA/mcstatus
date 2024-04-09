use anyhow::Result;
use figment::{
    providers::{Format, Json, Toml, Yaml},
    Figment,
};
use serde::Deserialize;
use std::{num::NonZeroU16, path::Path};

#[derive(Deserialize, Clone, Debug)]
pub struct Server {
    pub name: String,

    #[serde(flatten)]
    pub address: ServerAddress,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
#[serde(rename_all = "lowercase")]
pub enum ServerAddress {
    HostPort { host: String, port: NonZeroU16 },
    Address { address: String },
}

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub servers: Vec<Server>,
}

impl Config {
    pub fn parse<P: AsRef<Path>>(filename: P) -> Result<Config> {
        let filename = filename.as_ref();

        let ext = filename
            .extension()
            .ok_or_else(|| anyhow::anyhow!("file does not have an extension"))?
            .to_string_lossy();

        let mut figment = Figment::new();

        figment = match ext.to_lowercase().as_ref() {
            "json" => figment.merge(Json::file(filename)),
            "yml" | "yaml" => figment.merge(Yaml::file(filename)),
            "toml" => figment.merge(Toml::file(filename)),
            _ => anyhow::bail!("unsupported file format"),
        };

        Ok(figment.extract()?)
    }
}
