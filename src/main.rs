use anyhow::Result;
use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
};

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

    let mut buffer = String::new();
    let read_stream = stream.try_clone()?;
    let write_stream = stream;

    let mut reader = BufReader::new(read_stream);
    let mut writer = BufWriter::new(write_stream);

    let good_response = "HTTP/1.1 200 OK\r\n\r\n";
    let not_found_resp = "HTTP/1.1 404 Not Found\r\n\r\n";
    let bad_request_resp = "HTTP/1.1 400 Bad Request\r\n\r\n";

    reader.read_line(&mut buffer)?;

    let parts = buffer.split_whitespace().collect::<Vec<&str>>();

    let resp = match parts.get(1) {
        Some(request_path) => {
            if request_path.starts_with("/echo/") {
                let content = request_path.split('/').last().unwrap_or("");
                &format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    content.len(),
                    content
                )
            } else if *request_path == "/" {
                good_response
            } else {
                not_found_resp
            }
        }
        None => bad_request_resp,
    };

    writer.write_all(resp.as_bytes())?;

    Ok(())
}
