#[cfg(test)]
mod tests {
    use crate::http::request::parse;

    #[test]
    fn test_parse_request() {
        let request = String::from("GET / HTTP/1.1\r\nHost: www.example.com\r\n\r\n");
        let mut stream = request.as_bytes();

        let request = parse(&mut stream);

        assert_eq!(request.method, "GET");
        assert_eq!(request.location, "/");
    }
}
