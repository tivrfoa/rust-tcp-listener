use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

const BUFFER_LEN: usize = 512;

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
                thread::sleep(Duration::from_secs(2));

                // Create a buffer to store incoming data
                let mut buffer = [0; BUFFER_LEN];
                let mut total_bytes_received = 0;

                loop {
                    match stream.read(&mut buffer) {
                        Ok(bytes_read) => {
                            if bytes_read == 0 {
                                break;
                            }
                            println!("\n<<Received {bytes_read}>>");
                            total_bytes_received += bytes_read;

                            // Echo the received message back to the client
                            print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));

                            if bytes_read < BUFFER_LEN {
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed reading from stream: {e}");
                            break;
                        }
                    }
                }
                println!("Received total {total_bytes_received} bytes.\n");

                // Construct an HTTP/1.1 response
                let response = "HTTP/1.1 200 OK\r\n\
                                Content-Type: text/html; charset=UTF-8\r\n\
                                Content-Length: 13\r\n\
                                Connection: close\r\n\r\n\
                                Hello, world!";

                // Send the response back to the client
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("Failed wrting to stream: {e}");
                }
                stream.flush()?;
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
