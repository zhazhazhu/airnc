use std::process::exit;

use clap::Parser;
use colored::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    ///  Sets a custom file path
    pub path: Option<String>,
}

pub fn run_cli() -> Option<Cli> {
    let cli = Cli::parse();

    if cli.path.is_none() {
        eprintln!("{}", "🔺 File path can't null...".red());
        exit(1);
    }

    Some(cli)
}
