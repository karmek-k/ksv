#[cfg(test)]
mod tests {
    use crate::http::request::HttpRequest;

    #[test]
    fn test_parse_request() {
        let request = String::from("GET / HTTP/1.1\r\nHost: www.example.com\r\n\r\n");
        let mut stream = request.as_bytes();

        let request = HttpRequest::parse_stream(&mut stream).unwrap();

        assert_eq!(request.method, "GET");
        assert_eq!(request.location, "/");
    }

    #[test]
    fn test_parse_invalid_request_nonsense() {
        let request =
            String::from("sadsa GE543t46T ubobo HTTP/1.1\r\nHost:gulu:::www.example.com\r\n\r\n");
        let mut stream = request.as_bytes();

        let result = HttpRequest::parse_stream(&mut stream);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_request_short() {
        let request = String::from("POST");
        let mut stream = request.as_bytes();

        let result = HttpRequest::parse_stream(&mut stream);

        assert!(result.is_err());
    }
}
