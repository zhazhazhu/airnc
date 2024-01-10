mod cli;
mod server;
mod utils;

use cli::run_cli;
use server::run_server;

fn main() -> std::io::Result<()> {
    let cli = run_cli().unwrap();

    run_server(cli)
}
