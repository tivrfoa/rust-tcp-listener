use std::collections::HashMap;
use std::fmt;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

// mod thread_pool3;
// use crate::thread_pool3::ThreadPool;

const BUFFER_LEN: usize = 1 << 14; // 16384

#[derive(Debug, PartialEq)]
enum RequestType {
    GET,
    POST,
    PUT,
}

#[derive(Debug)]
enum HttpError {
    BodyBiggerLen,
    EmptyRequest,
    FailedParseHeaders,
    FailedReadStream,
    FewContentLength,
    FlushError,
    WritingError,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HttpError::BodyBiggerLen => write!(f, "BodyBiggerLen"),
            HttpError::EmptyRequest => write!(f, "EmptyRequest"),
            HttpError::FailedParseHeaders => write!(f, "FailedParseHeaders"),
            HttpError::FailedReadStream => write!(f, "FailedReadStream"),
            HttpError::FewContentLength => write!(f, "FewContentLength"),
            HttpError::FlushError => write!(f, "FlushError"),
            HttpError::WritingError => write!(f, "WritingError"),
        }
    }
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

    fn parse(stream: &mut TcpStream) -> Result<Self, HttpError> {
        // Create a buffer to store incoming data
        let mut buffer = [0; BUFFER_LEN];

        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    eprintln!("Empty request?!");
                    return Err(HttpError::EmptyRequest);
                }
                // Convert buffer to string to get the request
                let body_part = String::from_utf8_lossy(&buffer[..bytes_read]);
                let mut request_lines = body_part.lines();
                let mut request = match Self::parse_headers(&mut request_lines) {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("{e}");
                        write_400_response(stream, &e);
                        return Err(HttpError::FailedParseHeaders);
                    }
                };

                if request.content_length == 0 {
                    let response = get_200_response("OK");
                    if let Err(e) = stream.write_all(response.as_bytes()) {
                        eprintln!("Failed wrting to stream: {e}");
                        return Err(HttpError::WritingError);
                    }
                    if let Err(e) = stream.flush() {
                        eprintln!("Failed to flush response: {e}");
                        return Err(HttpError::FlushError);
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
                                eprintln!("Bytes read {bytes_read}");
                                if bytes_read == 0 {
                                    return Err(HttpError::FewContentLength);
                                }
                                body.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
                            }
                            Err(e) => {
                                eprintln!("rtb: Failed reading stream: {e}");
                                return Err(HttpError::FailedReadStream);
                            }
                        }
                    }
                    if body.len() > request.content_length {
                        dbg!(body);
                        return Err(HttpError::BodyBiggerLen);
                    }
                    request.body = Some(body);
                }
                Ok(request)
            }
            Err(e) => {
                eprintln!("rt: Failed reading stream: {e}");
                Err(HttpError::FailedReadStream)
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

fn handle_request(mut stream: TcpStream) {
    let request = Request::parse(&mut stream);
    dbg!(&request);

    match request {
        Ok(req) => {}
        Err(e) => {
            // close connection
            write_400_response(&mut stream, &e.to_string());
            return;
        }
    }

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
    if let Err(e) = stream.flush() {
        eprintln!("hr: Failed to flush: {e}");
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on 127.0.0.1:7878...");

    // Accept incoming connections in a loop
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr()?);
                std::thread::spawn(move || {
                    handle_request(stream);
                });
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

fn write_400_response(stream: &mut TcpStream, e: &str) -> Result<(), HttpError> {
    let response = get_400_response(e);
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed wrting to stream: {e}");
        return Err(HttpError::WritingError);
    }
    if let Err(e) = stream.flush() {
        eprintln!("{e}");
        return Err(HttpError::FlushError);
    }
    Ok(())
}
