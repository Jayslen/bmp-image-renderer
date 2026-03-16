use std::{
    fs::File,
    io::{BufReader, Read},
};

struct BMP {
    size: usize,
    starting_adress: usize,
}

impl BMP {
    fn get_size(&self) -> String {
        if self.size >= 1_048_576 {
            return format!("{:.1} MB", self.size as f64 / 1_048_576_f64);
        } else {
            return format!("{:.1} KB", self.size as f64 / 1024_f64);
        }
    }
}

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

    let image: BMP = BMP {
        size: u32::from_le_bytes(bmp_header[2..6].try_into().unwrap()) as usize,
        starting_adress: u32::from_le_bytes(bmp_header[10..].try_into().unwrap()) as usize,
    };

    println!("Size of image: {}", image.get_size());
    println!("Starting address of image data: {}", image.starting_adress);
    // get size of image
}
