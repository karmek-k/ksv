use std::io;

/// HTTP request abstraction.
pub struct HttpRequest {
    /// HTTP method.
    pub method: String,
    /// Request path.
    pub path: String,
}

impl HttpRequest {
    pub fn from(bytes: &[u8]) -> io::Result<Self> {
        Ok(HttpRequest {
            method: "GET".into(),
            path: "/".into(),
        })
    }
}
