use std::str;
use std::collections::HashMap;
use std::net::TcpStream;
use std::io::{BufReader, BufRead, Read};

pub struct HttpRequest {
    // Unused for now, hence leading underscore
    _raw_request: TcpStream,
    pub method: String,
    pub route: String,
    pub query_params: HashMap<String, String>,
    pub protocol_version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(PartialEq)]
enum RequestParserState {
    InMethod,
    InRoute,
    InQueryString,
    InVersion,
    InHeader,
    InBody,
    End
}

pub fn parse_raw_request(mut stream: TcpStream) -> std::io::Result<HttpRequest> {
    let mut reader = BufReader::new(&mut stream);
    let mut buffer: Vec<u8> = vec![];
    let mut query_params_buffer = String::new();
    let mut parse_state = RequestParserState::InMethod;

    let mut headers: HashMap<String, String> = HashMap::new();
    let mut query_params: HashMap<String, String> = HashMap::new();
    let mut body = String::new();

    let mut method = String::new();
    let mut route = String::new();
    let mut version = String::new();

    loop {
        match parse_state {
            RequestParserState::InMethod => {
                reader.read_until(b' ', &mut buffer)?;
                method = str::from_utf8(&buffer).unwrap().trim_end().to_string();
                parse_state = RequestParserState::InRoute;
            }
            RequestParserState::InRoute => {
                reader.read_until(b' ', &mut buffer)?;
                route = str::from_utf8(&buffer).unwrap().trim_end().to_string();
                if route.contains("?") {
                    let mut route_split: Vec<String> = route.split("?").map(|s| s.to_string()).collect();
                    query_params_buffer = route_split.pop().unwrap();
                    route = route_split.pop().unwrap();
                    parse_state = RequestParserState::InQueryString;
                } else {
                    parse_state = RequestParserState::InVersion;
                }
            }
            RequestParserState::InQueryString => {
                for param in query_params_buffer.split("&").collect::<Vec<_>>() {
                    // Malformed query string. Discard and continue
                    if param == "" {
                        break;
                    }
                    let mut param_parts = param.split("=").collect::<Vec<_>>();

                    // Unspecified value, default to empty string as per RFC
                    if param_parts.len() < 2 {
                        param_parts.push("");
                    }
                    query_params.insert(
                        param_parts[0].to_string(),
                        param_parts[1].to_string()
                    );
                }
                parse_state = RequestParserState::InVersion;
            }
            RequestParserState::InVersion => {
                reader.read_until(b'\n', &mut buffer)?;
                version = str::from_utf8(&buffer).unwrap().trim_end().to_string();
                parse_state = RequestParserState::InHeader;
            }
            RequestParserState::InHeader => {
                let mut header_buffer = String::new();
                reader.read_line(&mut header_buffer)?;

                if header_buffer == "\r\n" {
                    // Finished reading headers, hit separating blank line
                    parse_state = RequestParserState::InBody;
                } else {
                    // Still reading headers
                    let header_parts = header_buffer.split(": ").collect::<Vec<_>>();
                    headers.insert(
                        header_parts[0].to_string().to_lowercase(),
                        header_parts[1].trim_end().to_string()
                    );
                }
            }
            RequestParserState::InBody => {
                if headers.contains_key("content-length") {
                    let content_length = headers.get("content-length")
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    let mut body_buffer = vec![0u8; content_length];
                    reader.read_exact(&mut body_buffer)?;
                    body = str::from_utf8(&body_buffer).unwrap().to_string();
                }
                parse_state = RequestParserState::End;
            }
            RequestParserState::End => {}
        }
        buffer.clear();
        if parse_state == RequestParserState::End {
            break;
        }
    }

    Ok(HttpRequest {
        _raw_request: reader.into_inner().try_clone().unwrap(),
        method: method,
        route: route,
        query_params: query_params,
        protocol_version: version,
        headers: headers,
        body: body
    })
}
