use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ModemConfig {
    pub port: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub modem: ModemConfig,
}