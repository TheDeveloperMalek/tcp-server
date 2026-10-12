use std::{io, net::SocketAddr};

pub fn peer_label(address: Option<SocketAddr>) -> String {
    match address {
        Some(address) => address.to_string(),
        None => "unknown client".to_owned(),
    }
}

pub fn log_client_error(address: Option<SocketAddr>, error: &io::Error) {
    eprintln!("Error handling client {}: {}", peer_label(address), error);
}
