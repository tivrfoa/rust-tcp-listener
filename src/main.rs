use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

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
                thread::sleep(Duration::from_secs(3));

                // Create a buffer to store incoming data
                let mut buffer = [0; 512];
                let mut total_bytes_received = 0;

                loop {
                    match stream.read(&mut buffer) {
                        Ok(bytes_read) => {
                            if bytes_read == 0 {
                                break;
                            }
                            total_bytes_received += bytes_read;

                            // Echo the received message back to the client
                            print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                            // stream.write_all(&buffer[..bytes_read])?;
                            if let Err(e) = stream.write_all(b"Hi") {
                                eprintln!("Failed wrting to stream: {e}");
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed reading from stream: {e}");
                            break;
                        }
                    }
                }
                println!("Received total {total_bytes_received} bytes.\n");
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
