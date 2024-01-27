mod search;
mod util;
use image::{ImageBuffer, Rgb, RgbImage};

fn main() {
    let image_path = "../images/example_3/main.jpg";
    let parts_path = "../images/example_3/parts_1/";
    let extension = "jpg";
    let output_name = "example_3";

    let mut parts: Vec<RgbImage> = Vec::new();
    let image = util::load_image(image_path).expect("Error occured while loading main image!");
    let num_parts = util::count_parts(parts_path, extension);
    util::load_parts(&mut parts, parts_path, num_parts, extension);

    let image_pix: Vec<Vec<(u8, u8, u8)>> = util::image_to_pixel_matrix(&image);
    let mut output: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(image.width(), image.height());

    for part in parts.iter() {
        let part_pix = util::image_to_pixel_matrix(&part);
        let (best_x, best_y) = search::find_best_position(&image_pix, &part_pix);
        println!("({}, {})", best_x, best_y);
        util::generate_output(&mut output, &part, (best_x as u32, best_y as u32));
    }

    let output_path = format!("../results/{}.png", output_name);
    output
        .save(output_path)
        .expect("Failed to save assembled image!");
}
