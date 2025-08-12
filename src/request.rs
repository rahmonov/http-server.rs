use anyhow::Result;
use std::io::BufReader;
use std::{io::BufRead, net::TcpStream};

pub struct Request {
    pub method: String,
    pub path: String,
}

impl Request {
    pub fn new(method: String, path: String) -> Self {
        Request { method, path }
    }
}

pub fn parse_request(reader: &mut BufReader<TcpStream>) -> Result<Request> {
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;

    let mut parts = request_line.split_whitespace();

    let method = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing HTTP method"))?;

    let request_path = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing request path"))?;

    Ok(Request::new(method.to_string(), request_path.to_string()))
}
