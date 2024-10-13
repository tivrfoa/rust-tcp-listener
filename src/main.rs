use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

const BUFFER_LEN: usize = 512;

#[derive(Debug, PartialEq)]
enum RequestType {
    GET,
    POST,
    PUT,
}

#[derive(Debug)]
enum HttpError {
    BODY_BIGGER_LEN,
    EMPTY_REQUEST,
    WRITING_ERROR,
    FAILED_PARSE_HEADERS,
    FAILED_READ_STREAM,
    FEW_CONTENT_LENGTH,
    FLUSH_ERROR,
}

#[derive(Debug)]
struct Request {
    req_type: RequestType,
    path: String,
    content_length: usize,
    body: Option<String>,
    headers: HashMap<String, String>,
}

impl Request {
    fn new(
        req_type: RequestType,
        path: String,
        content_length: usize,
        body: Option<String>,
    ) -> Self {
        Self {
            req_type,
            path,
            content_length,
            body,
            headers: HashMap::new(),
        }
    }

    fn read_body(&mut self, request_lines: &String, stream: &mut TcpStream) -> Self {
        todo!()
    }

    fn parse(stream: &mut TcpStream) -> Result<Self, HttpError> {
        // Create a buffer to store incoming data
        let mut buffer = [0; BUFFER_LEN];

        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    eprintln!("Empty request?!");
                    return Err(HttpError::EMPTY_REQUEST);
                }
                // Convert buffer to string to get the request
                let body_part = String::from_utf8_lossy(&buffer[..]);
                let mut request_lines = body_part.lines();
                let mut request = match Self::parse_headers(&mut request_lines) {
                    Ok(r) => r,
                    Err(e) => {
                        let response = get_400_response(&e);
                        if let Err(e) = stream.write_all(response.as_bytes()) {
                            eprintln!("Failed wrting to stream: {e}");
                        }
                        if let Err(e) = stream.flush() {
                            eprintln!("{e}");
                            return Err(HttpError::FLUSH_ERROR);
                        }
                        return Err(HttpError::FAILED_PARSE_HEADERS);
                    }
                };

                if request.content_length == 0 {
                    let response = get_200_response("OK");
                    if let Err(e) = stream.write_all(response.as_bytes()) {
                        eprintln!("Failed wrting to stream: {e}");
                        return Err(HttpError::WRITING_ERROR);
                    }
                    if let Err(e) = stream.flush() {
                        eprintln!("Failed to flush response: {e}");
                        return Err(HttpError::FLUSH_ERROR);
                    }
                } else {
                    // read body
                    let mut body = String::with_capacity(request.content_length);

                    // read remaining lines, if any
                    for line in request_lines {
                        body.push_str(line);
                    }

                    while body.len() < request.content_length {
                        match stream.read(&mut buffer) {
                            Ok(bytes_read) => {
                                if bytes_read == 0 {
                                    return Err(HttpError::FEW_CONTENT_LENGTH);
                                }
                                body.push_str(&String::from_utf8_lossy(&buffer[..]));
                            }
                            Err(e) => {
                                eprintln!("rtb: Failed reading stream: {e}");
                                return Err(HttpError::FAILED_READ_STREAM);
                            }
                        }
                    }
                    if body.len() > request.content_length {
                        return Err(HttpError::BODY_BIGGER_LEN);
                    }
                    request.body = Some(body);
                }
                Ok(request)
            }
            Err(e) => {
                eprintln!("rt: Failed reading stream: {e}");
                Err(HttpError::FAILED_READ_STREAM)
            }
        }
    }

    fn parse_headers(lines: &mut std::str::Lines<'_>) -> Result<Self, String> {
        if let Some(request_line) = lines.next() {
            // The request line looks like: "GET / HTTP/1.1"
            let parts: Vec<&str> = request_line.split_whitespace().collect();
            if parts.len() >= 3 {
                let method = parts[0];
                let path = parts[1];

                let req_type = match method {
                    "GET" => RequestType::GET,
                    "POST" => RequestType::POST,
                    "PUT" => RequestType::PUT,
                    _ => return Err("Unsupported method".to_string()),
                };

                let mut request = Self::new(req_type, path.to_string(), 0, None);
                for line in lines {
                    if line.is_empty() {
                        // finished parsing headers
                        return Ok(request);
                    }
                    let Some((key, value)) = line.split_once(": ") else {
                        eprintln!("Invalid header: {line}");
                        return Err("Invalid header".to_string());
                    };
                    if value.is_empty() {
                        // TODO are empty headers valid?!
                        eprintln!("Empty header - key {key}");
                    }
                    if let Some(prev) = request.headers.insert(key.into(), value.into()) {
                        // TODO are repeated headers allowed?
                        eprintln!("Previous value for header {key} was {prev}");
                    }
                    if key == "Content-Length" {
                        if value.is_empty() {
                            return Err("Invalid Content-Length".to_string());
                        }

                        match value.parse::<usize>() {
                            Ok(l) => request.content_length = l,
                            Err(e) => {
                                return Err(format!("Invalid Content-Length: {}", e.to_string()))
                            }
                        }
                    }
                }
                Err("No empty line after headers".to_string())
            } else {
                eprintln!("Bad Request: {request_line}");
                Err("Invalid Request format".to_string())
            }
        } else {
            eprintln!("hr: Empty request");
            Err("Empty Request".to_string())
        }
    }
}

// Function to handle GET, POST, and PUT requests

fn main() -> std::io::Result<()> {
    // Bind the server to localhost:7878
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    println!("Server listening on 127.0.0.1:7878...");

    // Accept incoming connections in a loop
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr()?);

                let request = Request::parse(&mut stream);
                dbg!(request);

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
    format!(
        "HTTP/1.1 200 OK\r\n\
                                Content-Type: text/html; charset=UTF-8\r\n\
                                Content-Length: {len}\r\n\
                                Connection: close\r\n\r\n\
                                {arg}"
    )
}

fn get_400_response(error_message: &str) -> String {
    format!("HTTP/1.1 400 Bad Request\r\n\r\n{}", error_message)
}
