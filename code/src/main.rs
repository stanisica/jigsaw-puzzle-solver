mod search;
mod util;
use image::{ImageBuffer, Rgb, RgbImage};

fn main() {
    let image_path = "../images/example_6/main.png";
    let parts_path = "path/to/directory/";
    let extension = "jpg";
    let mut parts: Vec<RgbImage> = Vec::new();

    let image = util::load_image(image_path).expect("Error occured while loading main image!");
    let num_parts = util::count_parts(parts_path, extension);
    util::load_parts(&mut parts, parts_path, num_parts, extension);
    // EXAMPLE 1
    //util::load_parts_1_1(&mut parts, "../images/example_1/parts_1/");
    //util::load_parts_1_2(&mut parts, "../images/example_1/parts_2/");

    // EXAMPLE 2
    //util::load_parts_2_1(&mut parts, "../images/example_2/parts_1/");

    // EXAMPLE 3
    util::load_parts_1_1(&mut parts, "../images/example_1/parts_1/");
    // EXAMPLE 4

    // EXAMPLE 5

    // EXAMPLE 6

    util::load_parts_6_1(&mut parts, "../images/example_6/parts_1/");

    let image_pix: Vec<Vec<(u8, u8, u8)>> = util::image_to_pixel_matrix(&image);
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
