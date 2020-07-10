/*
 * Objectives:
 * 1. Bind TCP port
 * 2. Listen for messages
 * 3. Parse HTTP request contents
 * 4. direct request to a route (function)
 * 5. build response
 * 6. send response
 * 7. contine listening to requests
 */

use std::net::TcpListener;
use std::io::{BufReader, BufRead, Write};

fn main() -> std::io::Result<()> {
    // TODO: pass via parameter
    // question mark is a shorthand to a match statement, to handle side effect errors
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let mut buffer = String::new();
    let mut headers: Vec<String> = Vec::new();
    let mut headers_done = false;

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut reader = BufReader::new(&mut stream);

        // Method, Route, and HTTP Version
        reader.read_line(&mut buffer)?;

        // TODO: better handle part destructuring
        // There's a snippet with if let expression and pattern matching
        // Alternatively, use pattern matching and read by space delimiting
        let detail = buffer.to_string();
        let detail_parts = detail.split_whitespace().collect::<Vec<_>>();
        // Underscore variables denote they're unused at the moment
        // Will be used once method and version handling is implemented
        let _method = detail_parts[0];
        let route = detail_parts[1];
        let _version = detail_parts[2];

        // Headers
        // TODO: Use state enum and pattern matching instead
        while !headers_done {
            buffer.clear();
            reader.read_line(&mut buffer)?;
            if buffer == "\r\n" {
                headers_done = true;
            } else {
                headers.push(buffer.to_string());
            }
        }
        buffer.clear();
        // TODO: Handle body parsing
        // Read byte-per-byte until `content-length`

        // TODO:
        // Second iteration: export routes to functions
        // Third iteration: use a macro to define routes
        if route == "/" {
            let response = "HTTP/1.1 200 OK\r\n\r\nHello, user!\r\n";

            stream.write_all(response.as_bytes())?;
            stream.flush()?;
        }
    }

    Ok(())
}
