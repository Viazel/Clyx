mod cli;
mod commands;
mod config;
mod process;
mod templates;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    colored::control::set_override(!cli.no_color);
    commands::execute(cli)
}
