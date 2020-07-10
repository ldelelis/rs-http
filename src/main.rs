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

mod request;

use std::net::TcpListener;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // TODO: pass via parameter
    // question mark is a shorthand to a match statement, to handle side effect errors
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        let mut stream = stream?;
        let request: request::HttpRequest = request::parse_raw_request(stream.try_clone().unwrap());

        // TODO:
        // Second iteration: export routes to functions
        // Third iteration: use a macro to define routes
        if request.route == "/" {
            let response = "HTTP/1.1 200 OK\r\n\r\nHello, user!\r\n";

            stream.write_all(response.as_bytes())?;
            stream.flush()?;
        }
    }

    Ok(())
}
