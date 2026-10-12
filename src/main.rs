mod config;
mod constants;
mod server;
mod utils;

use config::ServerConfig;

fn main() {
    let config = ServerConfig::default();

    if let Err(error) = server::run(config) {
        eprintln!("Server failed to start or run: {error}");
        std::process::exit(1);
    }
}
