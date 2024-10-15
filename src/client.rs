use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
use std::time::Duration;

fn main() {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Could not connect to the server");
    
    // Set a read timeout to avoid hanging indefinitely
    stream.set_read_timeout(Some(Duration::from_secs(5))).expect("Failed to set read timeout");

    // First request with keep-alive
    let request1 = "GET / HTTP/1.1\r\nHost: localhost\r\nConnection: keep-alive\r\n\r\n";
    stream.write_all(request1.as_bytes()).expect("Failed to send request 1");
    println!("Sent first request");

    // Read response
    let mut buffer = [0; 512];
    let size = stream.read(&mut buffer).expect("Failed to read response 1");
    println!("Response 1: {}", str::from_utf8(&buffer[..size]).unwrap());

    // Second request over the same connection
    let request2 = "GET / HTTP/1.1\r\nHost: localhost\r\nConnection: keep-alive\r\n\r\n";
    stream.write_all(request2.as_bytes()).expect("Failed to send request 2");
    println!("Sent second request");

    // Read response
    let size = stream.read(&mut buffer).expect("Failed to read response 2");
    println!("Response 2: {}", str::from_utf8(&buffer[..size]).unwrap());

    // Third request to test keep-alive again
    let request3 = "GET / HTTP/1.1\r\nHost: localhost\r\nConnection: keep-alive\r\n\r\n";
    stream.write_all(request3.as_bytes()).expect("Failed to send request 3");
    println!("Sent third request");

    // Read response
    let size = stream.read(&mut buffer).expect("Failed to read response 3");
    println!("Response 3: {}", str::from_utf8(&buffer[..size]).unwrap());

    // Close the connection by sending a Connection: close header in a final request
    let close_request = "GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    stream.write_all(close_request.as_bytes()).expect("Failed to send close request");
    println!("Sent close request");

    // Read final response
    let size = stream.read(&mut buffer).expect("Failed to read final response");
    println!("Final Response: {}", str::from_utf8(&buffer[..size]).unwrap());
}

