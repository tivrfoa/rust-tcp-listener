use std::io::{Read, Write};
use std::net::TcpListener;

const BUFFER_LEN: usize = 512;

#[derive(PartialEq)]
enum RequestType {
    GET,
    POST,
    PUT,
}

// Function to handle GET, POST, and PUT requests
fn handle_request(request: &str) -> Result<(RequestType, String), String> {
    let mut lines = request.lines();
    if let Some(request_line) = lines.next() {
        // The request line looks like: "GET / HTTP/1.1"
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() >= 3 {
            let method = parts[0];
            let path = parts[1];

            match method {
                "GET" => Ok((RequestType::GET, path.into())),
                "POST" => Ok((RequestType::POST, path.into())),
                "PUT" => Ok((RequestType::PUT, path.into())),
                _ => Err("Unsupported method".to_string()),
            }
        } else {
            eprintln!("Bad Request: {request_line}");
            Err("Invalid Request format".to_string())
        }
    } else {
        eprintln!("hr: Empty request");
        Err("Empty Request".to_string())
    }
}

fn main() -> std::io::Result<()> {
    // Bind the server to localhost:7878
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    println!("Server listening on 127.0.0.1:7878...");

    // Accept incoming connections in a loop
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr()?);

                // Create a buffer to store incoming data
                let mut buffer = [0; BUFFER_LEN];
                let mut total_bytes_received = 0;

                // handle request type
                match stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        if bytes_read == 0 {
                            eprintln!("Empty request?!");
                            continue;
                        }
                        // Convert buffer to string to get the request
                        let request = String::from_utf8_lossy(&buffer[..]);

                        match handle_request(&request) {
                            Ok(req) => {
                                println!("Path: {}", req.1);
                                if req.0 == RequestType::GET {
                                    let response = get_200_response("OK");
                                    if let Err(e) = stream.write_all(response.as_bytes()) {
                                        eprintln!("Failed wrting to stream: {e}");
                                    }
                                    stream.flush()?;
                                    continue;
                                }
                            },
                            Err(e) => {
                                let response = get_400_response(&e);
                                if let Err(e) = stream.write_all(response.as_bytes()) {
                                    eprintln!("Failed wrting to stream: {e}");
                                }
                                stream.flush()?;
                                continue;
                            },
                        }
                    }
                    Err(e) => {
                        eprintln!("rt: Failed reading stream: {e}");
                        continue;
                    }
                }

                // Reading body
                loop {
                    match stream.read(&mut buffer) {
                        Ok(bytes_read) => {
                            if bytes_read == 0 {
                                break;
                            }
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

fn get_200_response(arg: &str) -> String {
    let len = arg.len();
    format!("HTTP/1.1 200 OK\r\n\
                                Content-Type: text/html; charset=UTF-8\r\n\
                                Content-Length: {len}\r\n\
                                Connection: close\r\n\r\n\
                                {arg}")
}

fn get_400_response(error_message: &str) -> String {
    format!("HTTP/1.1 400 Bad Request\r\n\r\n{}", error_message)
}
