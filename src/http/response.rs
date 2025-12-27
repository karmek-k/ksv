use super::status::Status;

/// HTTP response abstraction.
pub struct HttpResponse<'a> {
    /// Status code
    pub status: Status,
    /// Content type of the response
    pub content_type: &'a str,
    /// Response body
    pub body: String,
}

impl<'a> ToString for HttpResponse<'a> {
    /// Returns the HTTP response as a string.
    fn to_string(&self) -> String {
        let extra_length = 4;

        format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}\r\n\r\n",
            self.status.code(),
            self.status.message(),
            self.body.len() + extra_length,
            self.content_type,
            self.body
        )
    }
}

impl<'a> Default for HttpResponse<'a> {
    /// Returns a response with code 200, of type `text/plain` and with empty body.
    fn default() -> Self {
        Self {
            status: Status::Ok,
            content_type: "text/plain",
            body: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let res = HttpResponse {
            status: Status::Ok,
            content_type: "text/plain",
            body: String::from("Hello world"),
        };

        let res_string = res.to_string();

        assert!(res_string.contains("HTTP"));
        assert!(res_string.contains("200 OK"));
        assert!(res_string.contains("Content-Type: text/plain"));
        assert!(res_string.contains("Hello world"));
    }

    #[test]
    fn test_default() {
        let res = HttpResponse::default();

        assert_eq!(200, res.status.code());
        assert_eq!(String::new(), res.body);
        assert_eq!("text/plain", res.content_type);
    }
}
