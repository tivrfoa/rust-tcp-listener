use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
use std::time::Duration;

fn main() {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Could not connect to the server");
    
    // Set a read timeout to avoid hanging indefinitely
    stream.set_read_timeout(Some(Duration::from_secs(5))).expect("Failed to set read timeout");

	let mut buffer = [0; 512];
	for i in 1..=3 {
		println!("Sending {i} request");
		let request = "GET / HTTP/1.1\r\nHost: localhost\r\nConnection: keep-alive\r\n\r\n";
		stream.write_all(request.as_bytes()).expect("Failed to send request");
		println!("Sent {i} request");

		// Read response
		let size = stream.read(&mut buffer).expect("Failed to read response");
		println!("Response {i}: {}", str::from_utf8(&buffer[..size]).unwrap());
	}

    // Close the connection by sending a Connection: close header in a final request
    let close_request = "GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    stream.write_all(close_request.as_bytes()).expect("Failed to send close request");
    println!("Sent close request");

    // Read final response
    let size = stream.read(&mut buffer).expect("Failed to read final response");
    println!("Final Response: {}", str::from_utf8(&buffer[..size]).unwrap());
}

