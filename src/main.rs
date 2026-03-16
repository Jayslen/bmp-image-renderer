use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let file = File::open("./test_images/sample_640×426.bmp").unwrap();

    // read the first two bytes of the file to verify if its a bmp image
    // The first 2 must be "BM" for a bmp image
    let mut buffer_reader = BufReader::new(file);
    let mut bmp_header: [u8; 14] = [0; 14];

    buffer_reader
        .read_exact(&mut bmp_header[0..2])
        .expect("Error while reading the signature");

    let signature_chars = str::from_utf8(&bmp_header[0..2]).unwrap();

    if signature_chars != "BM" {
        panic!("This is not a bmp image")
    }

    buffer_reader
        .read_exact(&mut bmp_header[2..])
        .expect("Error while reading the header");

    // get size of image
    let bytes_size = u32::from_le_bytes(bmp_header[2..6].try_into().unwrap());
    let file_size;

    if bytes_size >= 1_048_576 {
        file_size = format!("{:.1} MB", bytes_size as f64 / 1_048_576_f64);
    } else {
        file_size = format!("{:.1} KB", bytes_size as f64 / 1024_f64);
    }
    println!("{:}", file_size)
}
