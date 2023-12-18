mod cli;
mod pargen;

use anyhow::Result as AnyResult;
use clap::Parser;

use cli::Cli;

fn main() -> AnyResult<()> {
    Cli::parse().run()?;

    Ok(())
}
