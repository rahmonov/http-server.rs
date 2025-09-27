use anyhow::Result;
use std::collections::HashMap;
use std::io::{BufReader, Read};
use std::{io::BufRead, net::TcpStream};

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn new(
        method: String,
        path: String,
        headers: HashMap<String, String>,
        body: Vec<u8>,
    ) -> Self {
        Request {
            method,
            path,
            headers,
            body,
        }
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

    let mut headers = HashMap::new();
    loop {
        let mut header_line = String::new();
        reader.read_line(&mut header_line)?;

        let header_line = header_line.trim();
        if header_line.is_empty() {
            break;
        }

        if let Some((name, value)) = header_line.split_once(":") {
            headers.insert(name.trim().to_string(), value.trim().to_string());
        } else {
            return Err(anyhow::anyhow!("Malformed header line: {}", header_line));
        }
    }

    // it means there is request body
    if let Some(content_length) = headers.get("Content-Length") {
        if let Ok(buf_len) = content_length.parse::<usize>() {
            let mut buffer = vec![0u8; buf_len];

            reader.read_exact(&mut buffer)?;

            return Ok(Request::new(
                method.to_string(),
                request_path.to_string(),
                headers,
                buffer,
            ));
        } else {
            return Err(anyhow::anyhow!(
                "Malformed Content-Length header: {content_length}"
            ));
        }
    } else {
        Ok(Request::new(
            method.to_string(),
            request_path.to_string(),
            headers,
            Vec::new(),
        ))
    }
}
