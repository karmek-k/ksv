#[cfg(test)]
mod tests {
    use crate::http::{response::HttpResponse, status::Status};

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