use super::status::Status;

pub struct HttpResponse {
    status: Status,
    content_type: String,
    body: String,
}

impl ToString for HttpResponse {
    fn to_string(&self) -> String {
        let (code, status_msg) = self.status.tuple();

        format!("HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}\r\n\r\n", code, status_msg, self.body.len(), self.content_type, self.body)
    }
}
