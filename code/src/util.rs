extern crate image;
use image::{open, ImageError, Rgb, RgbImage};

pub fn load_image(path: &str) -> Result<RgbImage, ImageError> {
    let img = open(path)?;
    Ok(img.to_rgb8())
}

pub fn load_parts(parts: &mut Vec<RgbImage>) {
    for i in 1..12 {
        let path = format!("../images/example_1/parts/part_{}.jpg", i);
        let error = format!("Error occurred while loading part_{}.jpg", i);
        let part = load_image(&path).expect(&error);
        parts.push(part);
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

pub fn print_pixel_matrix_portion(matrix: &Vec<Vec<(u8, u8, u8)>>, size: usize) {
    for row in matrix.iter().take(size) {
        for &pixel in row.iter().take(size) {
            print!("{:?} ", pixel);
        }
        println!();
    }
}
