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

            let bytes_per_pixel = bits_per_pixel as u32 / 8;

            let row_size = (bits_per_pixel as u32 * width + 31) / 32 * 4;
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
        Err(e) => {
            eprintln!("Something went wrong parsing the file. {:}", e);
        }
    }
}
