extern crate futures;
extern crate tokio_core;

use huawei_modem::{ HuaweiModem, cmd };
use tokio_core::reactor::Core;

use crate::config::Config;

pub async fn start_gate(config: Config) -> anyhow::Result<()> {
    log::info!("Opening modem at port {}...", config.modem.port.clone());

    let mut core = Core::new().unwrap();
    let mut modem = HuaweiModem::new_from_path(config.modem.port.clone(), &core.handle()).unwrap();

    let fut = cmd::sms::set_sms_textmode(&mut modem, true);
    println!("Result: {:?}", core.run(fut));
    // let fut = cmd::sms::send_sms_textmode(&mut modem, "+tel".to_string(), "Message".to_string());
    // println!("Result: {:?}", core.run(fut));

    Ok(())
}
