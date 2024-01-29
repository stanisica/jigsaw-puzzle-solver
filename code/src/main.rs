mod search;
mod util;
use image::{ImageBuffer, Rgb, RgbImage};
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let image_path = "../images/example_7/main.png";
    let parts_path = "../images/example_7/parts_1/";
    let parts_extension = "png";
    let output_name = "example_7";
    let mut parts: Vec<RgbImage> = Vec::new();

    let start = Instant::now();
    let image = util::load_image(image_path).expect("Error occured while loading main image!");
    let parts_num = util::count_parts(parts_path);
    //util::load_parts(&mut parts, parts_path, parts_num, parts_extension);
    util::load_parts_parallel(&mut parts, parts_path, parts_num, parts_extension);

    let image_pix: Vec<Vec<(u8, u8, u8)>> = util::image_to_pixel_matrix(&image);
    let mut output: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(image.width(), image.height());

    // Sequential
    // for part in parts.iter() {
    //     let part_pix = util::image_to_pixel_matrix(&part);
    //     let (best_x, best_y) = search::find_best_position(&image_pix, &part_pix);
    //     println!("({}, {})", best_x, best_y);
    //     util::generate_output(&mut output, &part, (best_x as u32, best_y as u32));
    // }

    // Parallel
    let results: Vec<_> = parts
        .par_iter()
        .map(|part| {
            let part_pix = util::image_to_pixel_matrix(&part);
            let (best_x, best_y) = search::find_best_position(&image_pix, &part_pix);
            (part, best_x, best_y)
        })
        .collect();

    for (part, best_x, best_y) in results {
        util::generate_output(&mut output, &part, (best_x as u32, best_y as u32));
    }
    //

    let output_path = format!("../results/{}.png", output_name);
    output
        .save(output_path)
        .expect("Failed to save assembled image!");

    let duration = start.elapsed();
    println!("Total elapsed time: {:?}", duration);
}
