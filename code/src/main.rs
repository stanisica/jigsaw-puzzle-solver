mod search;
mod util;
use image::{ImageBuffer, Rgb, RgbImage};

fn main() {
    let image = util::load_image("../images/example_1/main.jpg")
        .expect("Error occured while loading main image!");

    let mut parts: Vec<RgbImage> = Vec::new();
    util::load_parts(&mut parts, "../images/example_1/parts_1/");

    let image_pix: Vec<Vec<(u8, u8, u8)>> = util::image_to_pixel_matrix(&image);
    //util::print_pixel_matrix_portion(&image_pix, 4);

    let mut output: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(image.width(), image.height());
    for part in parts.iter() {
        let part_pix = util::image_to_pixel_matrix(&part);
        let (best_x, best_y) = search::find_best_position(&image_pix, &part_pix);

        util::generate_output(&mut output, &part, (best_x as u32, best_y as u32));
    }

    output
        .save("../results/output.png")
        .expect("Failed to save assembled image");
}
