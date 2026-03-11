use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let file = File::open("./test_images/text").unwrap();

    // read the first two bytes of the file to verify if its a bmp image
    // The first 2 must be "BM" for a bmp image
    let mut buffer_reader = BufReader::new(file);
    let mut signature_bytes: [u8; 2] = [0; 2];

    buffer_reader.read(&mut signature_bytes);

    let signature_chars = str::from_utf8(&signature_bytes).unwrap();

    if signature_chars == "BM" {
        println!("This is a bmp image");
    } else {
        println!("This is not a bmp image");
    }
}
