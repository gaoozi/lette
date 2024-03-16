use lette::cmd;

fn main() -> anyhow::Result<()> {
    let cli = cmd::setup()?;
    cli.handle()?;

    Ok(())
}
