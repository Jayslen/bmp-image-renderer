use std::{
    fs::File,
    io::{BufReader, Error, ErrorKind, Read, Seek, SeekFrom},
};

pub struct BMP {
    size: u32,
    width: u32,
    height: u32,
    bits_per_pixel: u16,
    compresion: u32,
    color_palette: Vec<[u8; 4]>,
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

    fn new(
        bmp_header: &[u8; 14],
        dib_header: &[u8; 40],
        pixel_array: Vec<u8>,
        color_palette: Vec<[u8; 4]>,
    ) -> BMP {
        BMP {
            size: read_le_bytes_u32(&bmp_header[2..6]),
            width: read_le_bytes_u32(&dib_header[4..8]),
            height: read_le_bytes_u32(&dib_header[8..12]),
            bits_per_pixel: read_le_bytes_u16(&dib_header[14..16]),
            compresion: read_le_bytes_u32(&dib_header[16..20]),
            color_palette,
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
    pub fn color_palette(&self) -> &Vec<[u8; 4]> {
        &self.color_palette
    }
}

pub fn parse_image(path: &String) -> Result<BMP, Error> {
    let file = File::open(path)
        .expect("Something went wrong parsing the file. Verify is the file specify exist");

    // read the first two bytes of the file to verify if its a bmp image
    // The first 2 must be "BM" for a bmp image
    let mut buffer_reader = BufReader::new(file);
    let mut bmp_header: [u8; 14] = [0; 14];

    buffer_reader.read_exact(&mut bmp_header)?;

    let signature_chars = str::from_utf8(&bmp_header[..2]).inspect_err(|e| eprintln!("{:}", Error::new(ErrorKind::InvalidData, format!("Something went wrong parsing the file. Verify is the file specify exist. {:?}", e)))).unwrap();

    if signature_chars != "BM" {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "This is not a bmp image",
        ));
    }

    let mut bitmap_info_header_buffer: [u8; 40] = [0; 40];
    buffer_reader.read_exact(&mut bitmap_info_header_buffer)?;

    let mut pixels: Vec<u8> = Vec::new();

    let pixels_adress = read_le_bytes_u32(&bmp_header[10..]);

    let palette_address = 14 + 40;
    let palette_size = pixels_adress - palette_address;
    let mut color_palette_buffer: Vec<u8> = vec![0; palette_size as usize];

    // read color palette
    // buffer_reader.seek(SeekFrom::Start(palette_address as u64))?;
    buffer_reader.read(&mut color_palette_buffer)?;

    let mut palette: Vec<[u8; 4]> = Vec::new();

    color_palette_buffer.chunks(4).for_each(|cp| {
        palette.push(cp.try_into().unwrap());
    });

    // println!("Color palette: {:?}", palettes);

    buffer_reader.seek(SeekFrom::Start(pixels_adress as u64))?;
    buffer_reader.read_to_end(&mut pixels)?;

    let image = BMP::new(&bmp_header, &bitmap_info_header_buffer, pixels, palette);

    println!("Bits per pixels {}", image.bits_per_pixel);
    println!("Starting adress {}", pixels_adress);
    println!("Compresion {}", image.compresion);
    println!("Width {}", image.width);
    println!("Height {}", image.height);
    println!("Size {}", image.get_size());

    let pixels_available: [u16; 6] = [1, 4, 8, 16, 24, 32];

    if image.compresion != 0 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "This image is compressed, this library only support uncompressed images",
        ));
    }

    if !pixels_available.contains(&image.bits_per_pixel) {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "This image has {} bits per pixel, this library only support 1, 4, 8, 16, 24 and 32 bits per pixel",
                image.bits_per_pixel
            ),
        ));
    }

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
