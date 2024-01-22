mod cli;
mod server;
mod utils;
mod ws;

use cli::run_cli;
use server::run_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = run_cli().unwrap();

    run_server(cli).await
}
