use std::process::exit;

use clap::Parser;
use colored::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    ///  Sets a custom file path
    pub path: Option<String>,
    #[clap(flatten)]
    pub config: AirncConfig,
}

#[derive(clap::Parser, Debug)]
pub struct AirncConfig {
    /// Whether to disable the link remote service
    #[clap(long = "service-disable", global = true)]
    pub service_disable: bool,
}

impl Default for AirncConfig {
    fn default() -> Self {
        Self {
            service_disable: false,
        }
    }
}

pub fn run_cli() -> Option<Cli> {
    let cli = Cli::parse();

    if cli.path.is_none() {
        eprintln!("{}", "🔺 File path can't null...".red());
        exit(1);
    }

    Some(cli)
}
