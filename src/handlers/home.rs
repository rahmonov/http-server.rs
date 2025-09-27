use std::collections::HashMap;

use crate::{request::Request, response::Response};

pub fn handle_home(_: &Request) -> Response {
    Response::new(200, HashMap::default(), None)
}
