use super::status::Status;

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
