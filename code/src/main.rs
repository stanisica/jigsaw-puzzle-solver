mod util;
use image::RgbImage;

fn main() {
    let image = util::load_image("../images/example_1/main.jpg")
        .expect("Error occured while loading main image!");

    let mut parts: Vec<RgbImage> = Vec::new();
    util::load_parts(&mut parts);

    let image_pix: Vec<Vec<(u8, u8, u8)>> = util::image_to_pixel_matrix(&image);
    util::print_pixel_matrix_portion(&image_pix, 4);

    let part_pix = util::image_to_pixel_matrix(&parts[0]);

    // let parth = part_pix.len();
    // let partw = part_pix[0].len();

    // let mainh = image_pix.len();
    // let mainw = image_pix[0].len();

    let parth = 5;
    let partw = 2;

    let mainh = 4;
    let mainw = 4;

    let mut window_move_count = 0;

    for y in (0..mainh).step_by(parth) {
        for x in (0..mainw).step_by(partw) {
            if y + parth <= mainh && x + partw <= mainw {
                let window = create_window(&image_pix, x, y, partw, parth);
                window_move_count += 1;
            }
        }
    }

    println!("The window has moved {} times.", window_move_count);
}

fn create_window(
    main_matrix: &Vec<Vec<(u8, u8, u8)>>,
    start_x: usize,
    start_y: usize,
    width: usize,
    height: usize,
) -> Vec<Vec<(u8, u8, u8)>> {
    let mut window = Vec::with_capacity(height);

    for y in start_y..start_y + height {
        let mut row = Vec::with_capacity(width);
        for x in start_x..start_x + width {
            row.push(main_matrix[y][x]);
        }
        window.push(row);
    }

    window
}
