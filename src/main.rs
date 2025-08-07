use anyhow::Result;
use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
};

fn main() -> Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

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

    reader.read_line(&mut buffer)?;

    let parts = buffer.split_whitespace().collect::<Vec<&str>>();
    let request_path = parts[1];

    if request_path == "/" {
        writer.write_all(good_response.as_bytes())?;
    } else {
        writer.write_all(not_found_resp.as_bytes())?;
    }

    println!("received request {parts:?}");

    Ok(())
}
