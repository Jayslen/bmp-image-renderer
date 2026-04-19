use bmp_renderer::{BMP, parse_image};
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let image_path = &args[1];

    let image = parse_image(&image_path);
    match image {
        Ok(image) => {
            render(&image);
        }
        Err(e) => {
            eprintln!("Something went wrong parsing the file. {:}", e);
        }
    }
}

fn render(image: &BMP) {
    let bits = image.bits_per_pixel();
    let width = image.width();
    let height = image.height();
    let pixel_array = image.pixel_array();
    let row_size = (bits as u32 * width + 31) / 32 * 4;

    if bits == 24 {
        let bytes_per_pixel = bits as u32 / 8;
        let row_by_pixels = width * bytes_per_pixel; // how many bytes in a row of pixels (without padding)

        for row in (0..height).rev() {
            let start = (row_size * row) as usize;
            let end = start + row_by_pixels as usize;

            for p in (start..end).step_by(bytes_per_pixel as usize) {
                print_color(pixel_array[p + 2], pixel_array[p + 1], pixel_array[p]);
            }
            print!("\n");
        }
    }

    if bits == 1 {
        let palette = image.color_palette();

        for row in (0..height).rev() {
            let start = (row_size * row) as usize;
            let end = start + row_size as usize;

            for p in start..end {
                for bit in (0..8).rev() {
                    let pixel_value = (pixel_array[p] >> bit) & 1;
                    let curr_palette = palette[pixel_value as usize];
                    print_color(curr_palette[2], curr_palette[1], curr_palette[0]);
                }
            }
            print!("\n");
        }
    }
}

fn print_color(r: u8, g: u8, b: u8) {
    print!("\x1b[48;2;{};{};{}m  \x1b[0m", r, g, b);
}
