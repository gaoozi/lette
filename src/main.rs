use dotenvy::dotenv;
use lette::{cmd, conf, log};

fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let _guard = log::setup();

    let cli = cmd::setup()?;
    let conf = conf::setup()?;

    cli.handle(&conf)?;

    Ok(())
}
