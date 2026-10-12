mod client;

use std::{io, net::TcpListener, thread};

use crate::{config::ServerConfig, utils};

pub fn run(config: ServerConfig) -> io::Result<()> {
    let listener = TcpListener::bind(&config.address)?;

    println!("TCP server listening on {}", config.address);

    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => {
                let address = stream.peer_addr().ok();

                let result =
                    thread::Builder::new()
                        .name("tcp-client".to_owned())
                        .spawn(move || {
                            if let Err(error) = client::handle_client(stream) {
                                utils::log_client_error(address, &error);
                            }
                        });

                if let Err(error) = result {
                    eprintln!("Failed to spawn client handler: {error}");
                }
            }
            Err(error) => {
                eprintln!("Failed to accept connection: {error}");
            }
        }
    }

    Ok(())
}
