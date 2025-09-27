use std::collections::HashMap;

use crate::{request::Request, response::Response};

pub fn handle_echo(request: &Request) -> Response {
    let content = request.path.split('/').last().unwrap_or("");
    Response::new(200, HashMap::default(), Some(content.to_string()))
}
