use clap::Parser;
use crate::{args::{CommandType, MeshtasticGateArgs}, config::Config, start::start_gate};

mod args;
mod config;
mod start;

#[tokio::main]
async fn main() {
    let res = run().await;
    match res {
        Err(err) => log::error!("Error: {}", err),
        Ok(_) => log::info!("Done"),
    }
}

async fn run() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    pretty_env_logger::env_logger::builder().init();

    let args = MeshtasticGateArgs::try_parse()?;
    log::debug!("Args: {:?}", args);

    let config_filename = std::env::var("MTG_CONFIG").unwrap_or("./config.yaml".to_string());
    log::debug!("Loading config {:?}...", &config_filename);
    let f = std::fs::File::open(config_filename)?;
    let config: Config = serde_yaml::from_reader(f)?;

    match args.command {
        CommandType::Start(_serve_command) => {
            start_gate(config).await?
        }
    }

    Ok(())
}
