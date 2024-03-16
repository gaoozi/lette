use clap::Args;

#[derive(Debug, Args)]
pub struct Cmd {
    /// Server port to listen on
    #[arg(
        short,
        long,
        value_name = "PORT",
        default_value = "8000"
    )]
    port: Option<u16>,
}

pub fn handle(_cmd: &Cmd)  -> anyhow::Result<()> {
    
    Ok(())
}

