use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
};

pub struct BMP {
    size: u32,
    starting_adress: u32,
    width: u32,
    height: u32,
    bits_per_pixel: u16,
    pixel_array: Vec<u8>,
}

impl BMP {
    pub fn get_size(&self) -> String {
        if self.size >= 1_048_576 {
            return format!("{:.1} MB", self.size as f64 / 1_048_576_f64);
        } else {
            return format!("{:.1} KB", self.size as f64 / 1024_f64);
        }
    }

    fn new(bmp_header: &[u8; 14], dib_header: &[u8; 40], pixel_array: Vec<u8>) -> BMP {
        BMP {
            size: read_le_bytes_u32(&bmp_header[2..6]),
            starting_adress: read_le_bytes_u32(&bmp_header[10..]),
            width: read_le_bytes_u32(&dib_header[4..8]),
            height: read_le_bytes_u32(&dib_header[8..12]),
            bits_per_pixel: read_le_bytes_u16(&dib_header[14..16]),
            pixel_array,
        }
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn bits_per_pixel(&self) -> u16 {
        self.bits_per_pixel
    }
    pub fn pixel_array(&self) -> &Vec<u8> {
        &self.pixel_array
    }
}

pub fn parse_image(path: &String) -> Result<BMP, ()> {
    let file = File::open(path)
        .expect("Something went wrong parsing the file. Verify is the file specify exist");

    // read the first two bytes of the file to verify if its a bmp image
    // The first 2 must be "BM" for a bmp image
    let mut buffer_reader = BufReader::new(file);
    let mut bmp_header: [u8; 14] = [0; 14];

    buffer_reader
        .read_exact(&mut bmp_header)
        .expect("Error while reading the header");

    let signature_chars = str::from_utf8(&bmp_header[..2]).unwrap();

    if signature_chars != "BM" {
        panic!("This is not a bmp image")
    }

    let mut bitmap_info_header_buffer: [u8; 40] = [0; 40];
    buffer_reader
        .read_exact(&mut bitmap_info_header_buffer)
        .expect("Errors while reading dib header");

    let mut pixels: Vec<u8> = Vec::new();

    let starting_adress = read_le_bytes_u32(&bmp_header[10..]);

    buffer_reader
        .seek(SeekFrom::Start(starting_adress as u64))
        .unwrap();

    buffer_reader.read_to_end(&mut pixels).unwrap();

    let image = BMP::new(&bmp_header, &bitmap_info_header_buffer, pixels);

    Ok(image)
}

fn read_le_bytes_u32(buffer: &[u8]) -> u32 {
    let bytes = buffer
        .try_into()
        .inspect_err(|e| eprintln!("Something went wrong {:?}", e))
        .unwrap();

    u32::from_le_bytes(bytes)
}

fn read_le_bytes_u16(buffer: &[u8]) -> u16 {
    let bytes = buffer
        .try_into()
        .inspect_err(|e| eprintln!("Something went wrong {:?}", e))
        .unwrap();

    u16::from_le_bytes(bytes)
}
