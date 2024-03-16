use anyhow::Ok;
use clap::Args;


#[derive(Debug, Args)]
pub struct Cmd {}

pub fn handle(_cmd: &Cmd) -> anyhow::Result<()> {
    println!("Hello, World!");
    Ok(())
}