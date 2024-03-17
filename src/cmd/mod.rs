mod create_admin;
mod hello;
mod migrate;
mod serve;

use clap::{Parser, Subcommand};

use crate::conf::Conf;

/// Lette app command line interface
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cmd {
    /// Specify alternative configuration file
    #[arg(short, long, value_name = "config", default_value = "config.toml")]
    pub config: Option<String>,

    #[command(subcommand)]
    pub subcmd: Option<Subcmd>,
}

#[derive(Debug, Subcommand)]
pub enum Subcmd {
    /// Hello world
    Hello(hello::Cmd),
    /// Start http server
    Serve(serve::Cmd),
    /// Run database migrations
    Migrate(migrate::Cmd),
    /// Create admin user
    CreateAdmin(create_admin::Cmd),
}

pub fn setup() -> anyhow::Result<Cmd> {
    let cmd = Cmd::parse();
    Ok(cmd)
}

impl Cmd {
    pub fn handle(&self, conf: &Conf) -> anyhow::Result<()> {
        match &self.subcmd {
            Some(Subcmd::Hello(subcmd)) => {
                hello::handle(subcmd, conf)?;
            }
            Some(Subcmd::Serve(subcmd)) => {
                serve::handle(subcmd, conf)?;
            }
            Some(Subcmd::Migrate(subcmd)) => {
                migrate::handle(subcmd, conf)?;
            }
            Some(Subcmd::CreateAdmin(subcmd)) => {
                create_admin::handle(subcmd, conf)?;
            }
            None => {}
        }
        Ok(())
    }
}
