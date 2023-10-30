use std::io::Read;

pub fn parse(stream: &mut dyn Read) {
    let mut buffer = [0; 2048];
    stream.read(&mut buffer).unwrap();
}
