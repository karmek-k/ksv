use std::io::Read;

pub struct HttpRequest {
    pub method: String,
    pub location: String,
}

pub fn parse(stream: &mut dyn Read) -> HttpRequest {
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

    println!("{:?}", strings);

    HttpRequest {
        method: strings[0].clone(),
        location: strings[1].clone(),
    }
}
