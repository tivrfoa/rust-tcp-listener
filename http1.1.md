## Explain what a server program needs to do to handle http 1.1

## ChatGPT:

To handle HTTP/1.1 in a server program, there are several key responsibilities that need to be met. HTTP/1.1 adds some features and requirements on top of the earlier HTTP/1.0 protocol, so your server needs to adhere to these rules in order to be compliant with the HTTP/1.1 specification. Let’s break down what needs to be done step by step.

### 1. **Establish a TCP connection**
   - **HTTP over TCP**: HTTP is an application-layer protocol that operates over TCP (Transmission Control Protocol). Your server needs to create and listen on a TCP socket. Each client connection will be a new TCP connection.
   - **Port**: HTTP usually uses port 80, but for testing or custom servers, you may use other ports like 8080 or 7878, as seen in the earlier Rust example.

### 2. **Receive HTTP requests**
   - **Handling the HTTP request line**: When a client connects, it sends an HTTP request that starts with a request line in the format:
     ```
     METHOD PATH HTTP/1.1
     ```
     Example:
     ```
     GET /index.html HTTP/1.1
     ```
   - **Read headers**: After the request line, there will be headers in the format of `Key: Value`. Your server needs to parse and store these headers. HTTP/1.1 requires that certain headers, such as `Host`, be present in the request.

### 3. **Support persistent connections by default (Keep-Alive)**
   - In HTTP/1.0, a new connection is opened for each request. In HTTP/1.1, the connection is persistent by default, meaning multiple requests and responses can be sent over the same connection.
   - **Keep-Alive**: You need to check whether the client wants to keep the connection alive by default (the client can explicitly close it by sending a `Connection: close` header). If the connection should remain open, you must continue to listen for more requests on the same connection until either side closes it.

### 4. **Handle HTTP methods**
   - Your server must recognize the request method (`GET`, `POST`, `PUT`, etc.) and respond appropriately. For example:
     - **GET**: Fetch a resource.
     - **POST**: Send data to be processed.
     - **PUT**: Update or create a resource.
   - Any unsupported methods should result in a `405 Method Not Allowed` response.

### 5. **Parse and handle HTTP headers**
   - **Host header**: HTTP/1.1 requires the `Host` header, which specifies the domain name of the server. Your server must reject requests without a `Host` header by returning `400 Bad Request`.
   - **Content-Length**: For requests that send data (like POST), your server needs to read the `Content-Length` header to determine how much data to read from the body of the request.
   - **Other common headers** include `User-Agent`, `Accept`, and `Connection`.

### 6. **Respond with a valid HTTP response**
   The server needs to respond with an HTTP/1.1-compliant response. This includes:
   - **Response status line**: The first line of the response must follow this format:
     ```
     HTTP/1.1 STATUS_CODE REASON_PHRASE
     ```
     Example:
     ```
     HTTP/1.1 200 OK
     ```
   - **Headers**: After the status line, send the appropriate headers. Common headers include:
     - `Content-Type`: Specifies the media type of the response body.
     - `Content-Length`: Specifies the size of the response body in bytes.
     - `Connection`: Specifies whether the connection should remain open or be closed (e.g., `Connection: keep-alive` or `Connection: close`).
   - **Response body**: If the response has a body (e.g., a web page), send it after the headers, separated by a blank line.

### 7. **Manage chunked transfer encoding**
   - In cases where the response size is not known ahead of time (e.g., streaming data), HTTP/1.1 allows the use of **chunked transfer encoding**. Your server can send the response in chunks, where each chunk is preceded by its size in hexadecimal format.

### 8. **Handle errors appropriately**
   - **Status codes**: The server must return the appropriate HTTP status codes, including:
     - `200 OK` for successful requests.
     - `400 Bad Request` if the request is malformed.
     - `404 Not Found` if the requested resource is not available.
     - `405 Method Not Allowed` if the request method is unsupported.
     - `500 Internal Server Error` for unexpected errors.
   - **Graceful error handling**: Even when errors occur, the server should still send a well-formed HTTP response so that the client can understand the error.

### 9. **Connection handling**
   - **Handle timeouts**: The server should close idle connections after a certain period of inactivity. If the client doesn’t send another request within this time, the server should clean up the connection to free up resources.
   - **Handle multiple concurrent connections**: In a real-world HTTP server, multiple clients will make requests at the same time. Your server should be able to handle multiple connections concurrently, typically using threads or asynchronous I/O.

### 10. **Other optional features in HTTP/1.1**
   - **Compression**: You can support content encoding (such as `gzip`) to compress the response body, reducing the size of the data sent.
   - **Caching**: Support for cache-related headers like `Cache-Control`, `ETag`, and `Last-Modified`.
   - **Redirection**: Handle HTTP redirection using `301 Moved Permanently`, `302 Found`, etc.

---

### Example: HTTP/1.1 Request and Response

#### Example of a basic GET request:
```
GET /index.html HTTP/1.1
Host: www.example.com
User-Agent: curl/7.64.1
Accept: */*
```

#### Example of a corresponding HTTP/1.1 response:
```
HTTP/1.1 200 OK
Content-Type: text/html
Content-Length: 137
Connection: keep-alive

<html>
<head><title>Welcome</title></head>
<body><h1>Hello, world!</h1></body>
</html>
```

### Summary

To handle HTTP/1.1, a server program must:
- Parse HTTP requests and extract method, path, and headers.
- Return HTTP/1.1-compliant responses, including status line, headers, and body.
- Support persistent connections with `Keep-Alive` unless instructed otherwise.
- Properly manage resources, concurrent connections, and error handling.

While writing a simple server from scratch helps with understanding these concepts, libraries such as **Hyper** (for Rust), **Netty** (for Java), and **libuv** (for C) handle many of these tasks automatically, making the process easier.