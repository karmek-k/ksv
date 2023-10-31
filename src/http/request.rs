use std::io::{self, Read};

/// A HTTP request abstraction.
pub struct HttpRequest {
    pub method: String,
    pub location: String,
}

impl HttpRequest {
    /// Construct a `HttpRequest` from a stream of bytes.
    pub fn parse_stream(stream: &mut dyn Read) -> io::Result<Self> {
        let mut buffer = [0; 2048];
        stream.read(&mut buffer).unwrap();

        // read while splitting by spaces
        let space_code: u8 = 32;

        let mut strings: Vec<String> = vec![];
        let mut string = String::new();
        for i in buffer {
            if i == space_code {
                strings.push(string.clone());
                string = String::new();
                continue;
            }

            string.push(i.into());
        }
        // identify parsed characters as method name

        // check method name (only letters)

        Ok(HttpRequest {
            method: strings[0].clone(),
            location: strings[1].clone(),
        })
    }
}
