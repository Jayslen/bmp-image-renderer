use bmp_renderer::parse_image;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let image_path = &args[1];

    let image = parse_image(&image_path);
    match image {
        Ok(image) => {
            let bits_per_pixel = image.bits_per_pixel();
            let width = image.width();
            let height = image.height();
            let pixel_array = image.pixel_array();
            render(bits_per_pixel, width, height, pixel_array);
        }
        Err(e) => {
            eprintln!("Something went wrong parsing the file. {:}", e);
        }
    }
}

fn render(bits: u16, width: u32, height: u32, pixel_array: &Vec<u8>) {
    if bits == 24 {
        let bytes_per_pixel = bits as u32 / 8;

        let row_size = (bits as u32 * width + 31) / 32 * 4;
        let row_by_pixels = width * bytes_per_pixel;

        for row in (0..height).rev() {
            let start = (row_size * row) as usize;
            let end = start + row_by_pixels as usize;

            for p in (start..end).step_by(bytes_per_pixel as usize) {
                print!(
                    "\x1b[48;2;{};{};{}m  \x1b[0m",
                    pixel_array[p + 2],
                    pixel_array[p + 1],
                    pixel_array[p]
                );
            }
            print!("\n");
        }
    }

    if bits == 1 {
        let row_size = (bits as u32 * width + 31) / 32 * 4;
        for row in (0..height).rev() {
            let start = (row_size * row) as usize;
            let end = start + row_size as usize;

            for p in start..end {
                for bit in (0..8).rev() {
                    let pixel_value = (pixel_array[p] >> bit) & 1;
                    if pixel_value == 1 {
                        print!("\x1b[48;2;255;255;255m  \x1b[0m");
                    } else {
                        print!("\x1b[48;2;0;0;0m  \x1b[0m");
                    }
                }
            }
            print!("\n");
        }
    }
}
