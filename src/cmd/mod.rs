mod hello;
mod serve;

use clap::{Parser, Subcommand};

/// Lette app command line interface
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cmd {
    /// Specify alternative configuration file
    #[arg(
        short,
        long,
        value_name = "config",
        default_value = "config.toml",
    )]
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
}

pub fn setup() -> anyhow::Result<Cmd> {
    let cmd = Cmd::parse();
    Ok(cmd)
}

impl Cmd {
    pub fn handle(&self) -> anyhow::Result<()> {
        match &self.subcmd {
            Some(Subcmd::Hello(subcmd)) => {
                hello::handle(subcmd)?;
            },
            Some(Subcmd::Serve(subcmd)) => {
                serve::handle(subcmd)?
            },
            None => {
                println!("not found this command");
            }
        }
        Ok(())
    }
}