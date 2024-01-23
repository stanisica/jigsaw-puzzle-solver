extern crate image;
use image::{open, ImageBuffer, ImageError, Rgb, RgbImage};

pub fn load_image(path: &str) -> Result<RgbImage, ImageError> {
    let img = open(path)?;
    Ok(img.to_rgb8())
}

pub fn load_parts_1(parts: &mut Vec<RgbImage>, path: &str) {
    for i in 1..13 {
        let path = format!("{}part_{}.jpg", path, i);
        println!("{}", path);
        let error = format!("Error occurred while loading part_{}.jpg", i);
        let part = load_image(&path).expect(&error);
        parts.push(part);
    }
}

pub fn load_parts_2(parts: &mut Vec<RgbImage>, path: &str) {
    for i in 1..=12 {
        for j in 1..=29 {
            let path = format!("{}image{}x{}.png", path, i, j);
            let part = load_image(&path).expect("ERROR");
            parts.push(part);
        }
    }
}

pub fn image_to_pixel_matrix(img: &RgbImage) -> Vec<Vec<(u8, u8, u8)>> {
    let (width, height) = img.dimensions();
    let mut matrix = Vec::with_capacity(height as usize);

    for y in 0..height {
        let mut row = Vec::with_capacity(width as usize);
        for x in 0..width {
            let Rgb([r, g, b]) = *img.get_pixel(x, y); //kopira vrednosti piksela u matricu
            row.push((r, g, b));
        }
        matrix.push(row);
    }
    matrix
}

pub fn print_matrix_portion(matrix: &Vec<Vec<(u8, u8, u8)>>, size: usize) {
    for row in matrix.iter().take(size) {
        for &pixel in row.iter().take(size) {
            print!("{:?} ", pixel);
        }
        println!();
    }
}

pub fn generate_output(
    assembled: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    part: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    position: (u32, u32),
) {
    let (pos_x, pos_y) = position;
    for y in 0..part.height() {
        for x in 0..part.width() {
            let pixel = part.get_pixel(x, y);
            if pos_x + x < assembled.width() && pos_y + y < assembled.height() {
                assembled.put_pixel(pos_x + x, pos_y + y, *pixel);
            }
        }
    }
}
