use anyhow::Result;
use clap::Parser;
use cli::Cli;
use colored::Colorize;

mod cli;

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {e}", "error:".red());
    }
}

fn run() -> Result<()> {
    let args = Cli::parse();
    Ok(())
}
