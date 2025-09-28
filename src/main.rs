use anyhow::Result;
use clap::Parser;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::{
    collections::HashMap,
    io::{BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
};

use crate::{
    args::Args,
    handlers::{echo::handle_echo, file::handle_file},
};
use crate::{handlers::home::handle_home, request::parse_request};
use crate::{handlers::user_agent::handle_user_agent, response::Response};
use codecrafters_http_server::ThreadPool;

pub mod args;
pub mod handlers;
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

    loop {
        let mut read_stream = BufReader::new(stream.try_clone()?);
        let mut write_stream = BufWriter::new(stream.try_clone()?);

        let request = match parse_request(&mut read_stream) {
            Ok(req) => req,
            Err(e) => {
                eprintln!("Client disconnected or invalid request: {:?}", e);
                break;
            }
        };

        let mut resp = if request.path.starts_with("/echo") {
            handle_echo(&request)
        } else if request.path == "/" {
            handle_home(&request)
        } else if request.path == "/user-agent" {
            handle_user_agent(&request)
        } else if request.path.starts_with("/files") {
            handle_file(&request, args)?
        } else {
            Response::new(404, HashMap::default(), Vec::new())
        };

        // should encode?
        if let Some(accept_encoding) = request.headers.get("Accept-Encoding") {
            if accept_encoding.contains("gzip") {
                resp.headers
                    .insert("Content-Encoding".to_string(), "gzip".to_string());

                if !resp.content.is_empty() {
                    let mut gzip_encoder = GzEncoder::new(Vec::new(), Compression::default());
                    gzip_encoder.write_all(&resp.content)?;
                    resp.content = gzip_encoder.finish()?;
                    resp.headers
                        .insert("Content-Length".to_string(), resp.content.len().to_string());
                }
            }
        }

        // do I close the connection?
        if request.headers.get("Connection").map(|s| s.to_lowercase()) == Some("close".into()) {
            resp.headers.insert("Connection".into(), "clone".into());
            write_stream.write(&resp.format())?;
            write_stream.flush()?;
            break; // client requested to close the connection
        } else {
            resp.headers
                .insert("Connection".into(), "keep-alive".into());
            write_stream.write(&resp.format())?;
            write_stream.flush()?;
        }
    }

    Ok(())
}
