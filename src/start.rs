use crate::config::Config;

pub async fn start_gate(config: Config) -> anyhow::Result<()> {
    log::info!("Opening modem at port {}...", config.modem.port.clone());

    Ok(())
}
