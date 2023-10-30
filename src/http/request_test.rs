#[cfg(test)]
mod tests {
    use std::io::{BufReader, Read};

    use crate::http::request::parse;

    #[test]
    fn test_parse_request() {
        let mut stream = BufReader::new("GET / HTTP/1.1\r\nHost: www.example.com\r\n\r\n");

        let request = parse(&mut stream);

        assert_eq!(request.method, "GET");
        assert_eq!(request.location, "/");
    }
}
