mod cli;
mod utils;
mod error;
mod generator;
mod template;

use cli::Cli;
use error::Result;
use clap::Parser;

fn main() ->Result<()>{
    let cli = Cli::parse();
    cli.run()?;
    Ok(())
}
