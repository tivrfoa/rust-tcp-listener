use std::io::{Read, Write};
use std::net::TcpListener;
use std::time::Duration;
use std::thread;

fn main() -> std::io::Result<()> {
    // Bind the server to localhost:7878
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    println!("Server listening on 127.0.0.1:7878...");

    // Accept incoming connections in a loop
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr()?);

				println!("Taking a nap");
				thread::sleep(Duration::from_secs(5));

                // Create a buffer to store incoming data
                let mut buffer = [0; 512];
                let bytes_read = stream.read(&mut buffer)?;

                if bytes_read > 0 {
                    // Echo the received message back to the client
                    println!("Received message: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
                    stream.write_all(&buffer[..bytes_read])?;
                }
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

