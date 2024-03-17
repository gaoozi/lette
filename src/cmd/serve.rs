use clap::Args;

use crate::{app, conf::Conf};

#[derive(Debug, Args)]
pub struct Cmd {
    /// Server port to listen on
    #[arg(short, long, value_name = "PORT", default_value = "8000")]
    port: Option<u16>,
}

pub fn handle(cmd: &Cmd, conf: &Conf) -> anyhow::Result<()> {
    let port = cmd.port.unwrap_or(conf.app.port);
    start_server(port, conf)?;
    Ok(())
}

fn start_server(port: u16, conf: &Conf) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move { app::serve(port, conf).await })?;

    std::process::exit(0)
}
