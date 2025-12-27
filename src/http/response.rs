use super::status::Status;

/// A HTTP response abstraction.
///
/// Implements the `ToString` trait, `to_string` returns
/// a valid HTTP response that can be sent back to the requester.
pub struct HttpResponse<'a> {
    pub status: Status,
    pub content_type: &'a str,
    pub body: String,
}

impl<'a> ToString for HttpResponse<'a> {
    fn to_string(&self) -> String {
        let (code, status_msg) = self.status.tuple();
        let extra_length = 4;

        format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}\r\n\r\n",
            code,
            status_msg,
            self.body.len() + extra_length,
            self.content_type,
            self.body
        )
    }
}

impl<'a> Default for HttpResponse<'a> {
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
}
