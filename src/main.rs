use anyhow::Result;
use clap::Parser;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
};

use crate::args::Args;
use crate::request::parse_request;
use crate::response::Response;
use codecrafters_http_server::ThreadPool;

pub mod args;
pub mod request;
pub mod response;

fn main() -> Result<()> {
    let args = Args::parse();

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream?;
        let args = args.clone();

        pool.execute(move || {
            handle_connection(stream, &args).unwrap();
        });
    }

    Ok(())
}

fn handle_connection(stream: TcpStream, args: &Args) -> Result<()> {
    println!("Handling a connection...");

    let file_dir = args.directory.as_deref().unwrap_or("");

    let read_stream = stream.try_clone()?;
    let write_stream = stream;

    let mut reader = BufReader::new(read_stream);
    let mut writer = BufWriter::new(write_stream);

    let request = parse_request(&mut reader)?;

    let resp = if request.path.starts_with("/echo") {
        let content = request.path.split('/').last().unwrap_or("");
        Response::new(200, HashMap::default(), Some(content.to_string()))
    } else if request.path == "/" {
        Response::new(200, HashMap::default(), None)
    } else if request.path == "/user-agent" {
        if let Some(user_agent) = request.headers.get("User-Agent") {
            Response::new(200, HashMap::default(), Some(user_agent.to_owned()))
        } else {
            Response::new(400, HashMap::default(), None)
        }
    } else if request.path.starts_with("/files") {
        let file_path = format!("{}{}", file_dir, request.path.trim_start_matches("/files"));

        if request.method == "GET" {
            if let Ok(content) = fs::read(file_path) {
                Response::new(
                    200,
                    HashMap::from([(
                        "Content-Type".to_string(),
                        "application/octet-stream".to_string(),
                    )]),
                    Some(String::from_utf8(content)?),
                )
            } else {
                Response::new(404, HashMap::default(), None)
            }
        } else {
            let mut file = File::create(file_path)?;
            file.write_all(&request.body)?;
            Response::new(201, HashMap::default(), None)
        }
    } else {
        Response::new(404, HashMap::default(), None)
    };

    writer.write_all(resp.as_string().as_bytes())?;

    Ok(())
}
