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
    color_palette: Option<Vec<[u8; 4]>>,
    pixel_array: Vec<u8>,
    row_pixel_size: u32,
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
        width: u32,
        height: u32,
        bits_per_pixel: u16,
        row_pixel_size: u32,
        pixel_array: Vec<u8>,
        color_palette: Option<Vec<[u8; 4]>>,
    ) -> BMP {
        BMP {
            size: read_le_bytes_u32(&bmp_header[2..6]),
            width,
            height,
            bits_per_pixel,
            compresion: read_le_bytes_u32(&dib_header[16..20]),
            color_palette,
            pixel_array,
            row_pixel_size,
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
    pub fn color_palette(&self) -> &Option<Vec<[u8; 4]>> {
        &self.color_palette
    }
    pub fn row_pixel_size(&self) -> u32 {
        self.row_pixel_size
    }
}
pub fn parse_bmp(path: &String) -> Result<BMP, Error> {
    let file = File::open(path)
        .expect("Something went wrong parsing the file. Verify is the file specify exist");

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

    let dib_header = parse_dbi_header(&mut buffer_reader)?;

    let pixels_available: [u16; 6] = [1, 4, 8, 16, 24, 32];

    let bits_per_pixel = read_le_bytes_u16(&dib_header[14..16]);
    let compresion = read_le_bytes_u32(&dib_header[16..20]);

    if compresion != 0 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "This image is compressed, this library only support uncompressed images",
        ));
    }

    if !pixels_available.contains(&bits_per_pixel) {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!(
                "This image has {} bits per pixel, this library only support 1, 4, 8, 16, 24 and 32 bits per pixel",
                &bits_per_pixel
            ),
        ));
    }

    let pixels_adress = read_le_bytes_u32(&bmp_header[10..]);

    let width = read_le_bytes_u32(&dib_header[4..8]);
    let height = read_le_bytes_u32(&dib_header[8..12]);
    let color_palette = parse_color_palette(&mut buffer_reader, &pixels_adress, &bits_per_pixel);
    let pixels_data = parse_pixels(
        &mut buffer_reader,
        &pixels_adress,
        &width,
        &height,
        &bits_per_pixel,
    )?;
    let pixel_row_size = (bits_per_pixel as u32 * width) / 8;

    let bmp = BMP::new(
        &bmp_header,
        &dib_header,
        width,
        height,
        bits_per_pixel,
        pixel_row_size,
        pixels_data,
        color_palette,
    );

    Ok(bmp)
}

fn parse_color_palette(
    image_buffer: &mut BufReader<File>,
    pixels_adress: &u32,
    bits_per_pixels: &u16,
) -> Option<Vec<[u8; 4]>> {
    if bits_per_pixels > &8 {
        return None;
    }

    let palette_address = 14 + 40;
    let palette_size = pixels_adress - palette_address;
    let mut color_palette_buffer: Vec<u8> = vec![0; palette_size as usize];

    image_buffer.read(&mut color_palette_buffer).ok();

    let mut palette: Vec<[u8; 4]> = Vec::new();

    color_palette_buffer.chunks(4).for_each(|cp| {
        palette.push(cp.try_into().unwrap());
    });

    Some(palette)
}

fn parse_dbi_header(image_buffer: &mut BufReader<File>) -> Result<[u8; 40], Error> {
    let mut bitmap_info_header_buffer: [u8; 40] = [0; 40];
    image_buffer.read_exact(&mut bitmap_info_header_buffer)?;

    Ok(bitmap_info_header_buffer)
}

fn parse_pixels(
    image_buffer: &mut BufReader<File>,
    pixels_adress: &u32,
    width: &u32,
    height: &u32,
    bits_per_pixel: &u16,
) -> Result<Vec<u8>, Error> {
    let mut pixels_buf: Vec<u8> = Vec::new();
    let mut pixels: Vec<u8> = Vec::new();

    image_buffer.seek(SeekFrom::Start(*pixels_adress as u64))?;
    image_buffer.read_to_end(&mut pixels_buf)?;

    let row_size = (width * *bits_per_pixel as u32 + 31) / 32 * 4;
    let bytes_per_row_pixels = width * *bits_per_pixel as u32 / 8;

    for row in (0..*height).rev() {
        let start = (row_size * row) as usize;
        let end = start + bytes_per_row_pixels as usize;

        pixels.extend_from_slice(&pixels_buf[start..end]);
    }

    drop(pixels_buf);

    Ok(pixels)
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
