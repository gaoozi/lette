use clap::Args;

use crate::conf::Conf;

#[derive(Debug, Args)]
pub struct Cmd {
    /// Email for admin user
    #[arg(short, long, value_name = "EMAIL", default_value = "test@123.com")]
    email: Option<String>,
    /// Password for admin user
    #[arg(short, long, value_name = "PASSWORD", default_value = "Pa$$wd123")]
    password: Option<String>,
}

pub fn handle(_cmd: &Cmd, _conf: &Conf) -> anyhow::Result<()> {
    Ok(())
}
