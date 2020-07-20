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

use std::collections::HashMap;
use std::env;
use std::net::TcpListener;
use std::io::Write;

fn parse_args(args: Vec<String>) -> std::io::Result<HashMap<String, String>> {
    let mut options: HashMap<String, String> = HashMap::new();
    let mut chunked_args = args.chunks_exact(2);

    loop {
        let next_chunk = chunked_args.next();
        if next_chunk.is_none() {
            break;
        }
        // Convert arguments from String to str in order to pattern match
        let unwrapped_chunk: Vec<_> = next_chunk.unwrap()
            .iter()
            .map(|s| s.as_str())
            .collect();

        match unwrapped_chunk.as_slice() {
            ["-o", value] => {
                options.insert("address".to_owned(), value.to_string());
            }
            ["-p", value] => {
                options.insert("port".to_owned(), value.to_string());
            }
            _ => unreachable!(),
        }
    }

    Ok(options)
}

fn main() -> std::io::Result<()> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let options: HashMap<String, String> = parse_args(args)?;

    let connection_string = &format!(
        "{}:{}",
        options.get("address").unwrap_or(&"127.0.0.1".to_string()),
        options.get("port").unwrap_or(&"8080".to_string())
    );

    let listener = TcpListener::bind(connection_string)?;

    for stream in listener.incoming() {
        let stream = stream?;
        let mut write_stream = stream.try_clone().unwrap();
        let request: request::HttpRequest = request::parse_raw_request(stream)?;

        // TODO:
        // Second iteration: export routes to functions
        // Third iteration: use a macro to define routes
        if request.route == "/" {
            let response = "HTTP/1.1 200 OK\r\n\r\nHello, user!\r\n";

            write_stream.write_all(response.as_bytes())?;
            write_stream.flush()?;
        }
    }

    Ok(())
}
