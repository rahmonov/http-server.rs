use std::collections::HashMap;

#[derive(Debug)]
pub struct Response {
    code: i32,
    headers: HashMap<String, String>,
    content: Option<String>,
}

impl Response {
    pub fn new(code: i32, mut headers: HashMap<String, String>, content: Option<String>) -> Self {
        if let Some(c) = &content {
            headers.insert("Content-Length".to_string(), c.len().to_string());

            if !headers.contains_key("Content-Type") {
                headers.insert("Content-Type".to_string(), "text/plain".to_string());
            }
        }

        Self {
            code,
            headers,
            content,
        }
    }

    pub fn as_string(&self) -> String {
        // status line
        let mut resp = format!("HTTP/1.1 {} {}\r\n", self.code, self.get_reason());

        // headers
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| format!("{k}: {v}"))
            .collect::<Vec<_>>()
            .join("\r\n");
        resp.push_str(&format!("{headers}\r\n\r\n"));

        // content
        if let Some(content) = &self.content {
            resp.push_str(content);
        }

        resp.to_string()
    }

    fn get_reason(&self) -> String {
        let reason = match self.code {
            200 => "OK",
            400 => "Bad Request",
            404 => "Not Found",
            _ => "Invalid Reason",
        };

        reason.to_string()
    }
}
