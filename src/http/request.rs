use std::string::FromUtf8Error;

/// HTTP request abstraction.
pub struct HttpRequest {
    /// HTTP method.
    pub method: String,
    /// Request path.
    pub path: String,
}

impl HttpRequest {
    /// Creates a HttpRequest from a UTF-8 slice of bytes.
    pub fn from(bytes: &[u8]) -> Result<Self, FromUtf8Error> {
        let body = String::from_utf8(bytes.into())?;
        let lines: Vec<_> = body.lines().collect();

        let first_line_tokens = lines[0].split(" ").collect::<Vec<_>>();
        let method = first_line_tokens[0].to_string();
        let path = first_line_tokens[1].to_string();

        Ok(HttpRequest { method, path })
    }
}
