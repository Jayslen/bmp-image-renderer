use bmp_renderer::{BMP, parse_bmp};
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let image_path = &args[1];

    let image = parse_bmp(&image_path);
    match image {
        Ok(image) => {
            println!("Bits per pixels {}", image.bits_per_pixel());
            println!("Width {}", image.width());
            println!("Height {}", image.height());
            println!("Size {}", image.get_size());
            render(&image);
        }
        Err(e) => {
            eprintln!("Something went wrong parsing the file. {:}", e);
        }
    }
}

fn render(image: &BMP) {
    let bits = image.bits_per_pixel();
    let height = image.height();
    let pixel_array = image.pixel_array();
    let row_pixel_size = image.row_pixel_size();

    if bits == 24 {
        let bytes_per_pixel = bits as u32 / 8;

        for row in 0..height {
            let start = (row_pixel_size * row) as usize;
            let end = start + row_pixel_size as usize;

            for p in (start..end).step_by(bytes_per_pixel as usize) {
                print_color(pixel_array[p + 2], pixel_array[p + 1], pixel_array[p]);
            }
            print!("\n");
        }
    }

    if bits == 16 {
        let bytes_per_pixel = bits as u32 / 8;

        for row in 0..height {
            let start = (row_pixel_size * row) as usize;
            let end = start + row_pixel_size as usize;

            for p in (start..end).step_by(bytes_per_pixel as usize) {
                let low = pixel_array[p] as u16;
                let high = pixel_array[p + 1] as u16;
                let pixel = (high << 8) | low;
                let r = ((pixel >> 10) & 0x1F) * 255 / 31;
                let g = ((pixel >> 5) & 0x1F) * 255 / 31;
                let b = (pixel & 0x1F) * 255 / 31;
                print_color(r as u8, g as u8, b as u8);
            }
            print!("\n");
        }
    }

    if bits == 4 || bits == 1 || bits == 8 {
        let palette = image.color_palette();
        println!("{:?}", palette);

        for row in 0..height {
            let start = (row_pixel_size * row) as usize;
            let end = start + row_pixel_size as usize;

            let mut mask = 0x1;

            if bits == 4 {
                mask = 0xF;
            } else if bits == 8 {
                mask = 0xFF;
            }

            for p in start..end {
                for bit in (0..8).step_by(bits as usize).rev() {
                    let pixel_value = (pixel_array[p] >> bit) & mask;
                    if let Some(palette) = palette {
                        let curr_palette = palette[pixel_value as usize];
                        print_color(curr_palette[2], curr_palette[1], curr_palette[0]);
                    }
                }
            }
            print!("\n");
        }
    }
}

fn print_color(r: u8, g: u8, b: u8) {
    print!("\x1b[48;2;{};{};{}m  \x1b[0m", r, g, b);
}
