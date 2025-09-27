use std::collections::HashMap;

use crate::{request::Request, response::Response};

pub fn handle_user_agent(request: &Request) -> Response {
    if let Some(user_agent) = request.headers.get("User-Agent") {
        Response::new(200, HashMap::default(), Some(user_agent.to_owned()))
    } else {
        Response::new(400, HashMap::default(), None)
    }
}
