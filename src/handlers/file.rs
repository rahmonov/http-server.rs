use anyhow::Result;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};

use crate::{args::Args, request::Request, response::Response};

pub fn handle_file(request: &Request, args: &Args) -> Result<Response> {
    let file_dir = args.directory.as_deref().unwrap_or("");
    let file_path = format!("{}{}", file_dir, request.path.trim_start_matches("/files"));

    let res = if request.method == "GET" {
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
    };

    Ok(res)
}
