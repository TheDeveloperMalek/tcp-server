use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

use crate::constants::{BUFFER_SIZE, DEFAULT_RESPONSE};

pub(super) fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0_u8; BUFFER_SIZE];

    let bytes_read = stream.read(&mut buffer)?;

    if bytes_read == 0 {
        return Ok(());
    }

    let message = String::from_utf8_lossy(&buffer[..bytes_read]);

    println!("Received: {}", message.trim_end());

    stream.write_all(DEFAULT_RESPONSE)?;
    stream.flush()?;

    Ok(())
}
