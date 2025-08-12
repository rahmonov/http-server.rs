use anyhow::Result;
use std::{
    collections::HashMap,
    io::{BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
};

use crate::request::parse_request;
use crate::response::Response;

pub mod request;
pub mod response;

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
