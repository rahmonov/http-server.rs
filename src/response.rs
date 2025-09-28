use std::collections::HashMap;

#[derive(Debug)]
pub struct Response {
    code: i32,
    pub headers: HashMap<String, String>,
    pub content: Vec<u8>,
}

impl Response {
    pub fn new(code: i32, mut headers: HashMap<String, String>, content: Vec<u8>) -> Self {
        headers.insert("Content-Length".to_string(), content.len().to_string());

        if !headers.contains_key("Content-Type") {
            headers.insert("Content-Type".to_string(), "text/plain".to_string());
        }

        Self {
            code,
            headers,
            content,
        }
    }

    pub fn format(&self) -> Vec<u8> {
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

        let mut bytes = resp.into_bytes();

        bytes.extend_from_slice(&self.content);

        bytes
    }

    fn get_reason(&self) -> String {
        let reason = match self.code {
            200 => "OK",
            201 => "Created",
            400 => "Bad Request",
            404 => "Not Found",
            _ => "Invalid Reason",
        };

        reason.to_string()
    }
}
