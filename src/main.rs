use anyhow::Result;
use clap::Parser;
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

    let read_stream = stream.try_clone()?;
    let write_stream = stream;

    let mut reader = BufReader::new(read_stream);
    let mut writer = BufWriter::new(write_stream);

    let request = parse_request(&mut reader)?;

    let mut resp = if request.path.starts_with("/echo") {
        handle_echo(&request)
    } else if request.path == "/" {
        handle_home(&request)
    } else if request.path == "/user-agent" {
        handle_user_agent(&request)
    } else if request.path.starts_with("/files") {
        handle_file(&request, args)?
    } else {
        Response::new(404, HashMap::default(), None)
    };

    if let Some(accept_encoding) = request.headers.get("Accept-Encoding") {
        if accept_encoding.contains("gzip") {
            resp.headers
                .insert("Content-Encoding".to_string(), "gzip".to_string());
        }
    }

    writer.write_all(resp.as_string().as_bytes())?;

    Ok(())
}
