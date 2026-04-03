use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
};

struct BMP {
    size: u32,
    starting_adress: u32,
    width: u32,
    height: u32,
    bits_per_pixel: u16,
    pixels_size: u32,
}

impl BMP {
    fn get_size(&self) -> String {
        if self.size >= 1_048_576 {
            return format!("{:.1} MB", self.size as f64 / 1_048_576_f64);
        } else {
            return format!("{:.1} KB", self.size as f64 / 1024_f64);
        }
    }

    fn init_info(bmp_header: &[u8; 14], dib_header: &[u8; 40]) -> BMP {
        BMP {
            size: read_le_bytes_u32(&bmp_header[2..6]),
            starting_adress: read_le_bytes_u32(&bmp_header[10..]),
            width: read_le_bytes_u32(&dib_header[4..8]),
            height: read_le_bytes_u32(&dib_header[8..12]),
            bits_per_pixel: read_le_bytes_u16(&dib_header[14..16]),
            pixels_size: read_le_bytes_u32(&dib_header[20..24]),
        }
    }
}

fn main() {
    let file = File::open("./test_images/handcrafted_square.bmp").unwrap();

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

    let image = BMP::init_info(&bmp_header, &bitmap_info_header_buffer);

    buffer_reader.seek(SeekFrom::Start((image.starting_adress as u64)));

    let row_size = (image.bits_per_pixel as u32 * image.width + 31) / 32 * 4;

    for row in (1..=image.height).rev() {
        let row_to_print = (row * row_size) - row_size + image.starting_adress;
        // println!("{}", row_to_print);
        buffer_reader.seek(SeekFrom::Start((row_to_print as u64)));
        for _ in 0..image.width {
            let mut pixels: [u8; 3] = [0; 3];
            buffer_reader.read_exact(&mut pixels).unwrap();

            print!(
                "\x1b[48;2;{};{};{}m  \x1b[0m",
                pixels[2], pixels[1], pixels[0]
            );
        }

        // skip padding
        let padding = row_size as usize - (image.width as usize * 3);
        let mut padding_bytes = vec![0u8; padding];
        buffer_reader.read_exact(&mut padding_bytes).unwrap();

        print!("\n");
    }
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
