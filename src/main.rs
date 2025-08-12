use anyhow::Result;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
};

pub struct Response {
    code: i32,
    headers: HashMap<String, String>,
    content: Option<String>,
}

impl Response {
    pub fn new(code: i32, mut headers: HashMap<String, String>, content: Option<String>) -> Self {
        if let Some(c) = &content {
            headers.insert("Content-Length".to_string(), c.len().to_string());
            headers.insert("Content-Type".to_string(), "text/plain".to_string());
        }

        Self {
            code,
            headers,
            content,
        }
    }

    pub fn as_string(&self) -> String {
        // status line
        let mut resp = format!("HTTP/1.1 {} {}\r\n", self.code, self.get_reason());

        // headers
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| format!("{k}: {v}"))
            .collect::<Vec<_>>()
            .join("\r\n");
        resp.push_str(&format!("{headers}\r\n\r\n"));

        // content
        if let Some(content) = &self.content {
            resp.push_str(content);
        }

        resp.to_string()
    }

    fn get_reason(&self) -> String {
        let reason = match self.code {
            200 => "OK",
            400 => "Bad Request",
            404 => "Not Found",
            _ => "Invalid Reason",
        };

        reason.to_string()
    }
}

pub struct Request {
    method: String,
    path: String,
}

impl Request {
    pub fn new(method: String, path: String) -> Self {
        Request { method, path }
    }
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream)?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}

fn parse_request(reader: &mut BufReader<TcpStream>) -> Result<Request> {
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

fn handle_connection(stream: TcpStream) -> Result<()> {
    println!("Handling a connection...");

    let read_stream = stream.try_clone()?;
    let write_stream = stream;

    let mut reader = BufReader::new(read_stream);
    let mut writer = BufWriter::new(write_stream);

    let request = parse_request(&mut reader)?;

    let resp = match request.path {
        path if path.starts_with("/echo/") => {
            let content = path.split('/').last().unwrap_or("");
            Response::new(200, HashMap::default(), Some(content.to_string()))
        }
        path if path == "/" => Response::new(200, HashMap::default(), None),
        _ => Response::new(404, HashMap::default(), None),
    };

    println!("response: {:?}", resp.as_string());

    writer.write_all(resp.as_string().as_bytes())?;

    Ok(())
}
