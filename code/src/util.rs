extern crate image;
use image::{open, ImageBuffer, ImageError, Rgb, RgbImage};
use std::fs;
use std::path::Path;

pub fn load_image(path: &str) -> Result<RgbImage, ImageError> {
    let img = open(path)?;
    Ok(img.to_rgb8())
}

pub fn load_parts(parts: &mut Vec<RgbImage>, path: &str, num_parts: usize, extension: &str) {
    for i in 0..num_parts {
        let path = format!("{}image{}.{}", path, i, extension);
        let error = format!("Error occurred while loading image{}.{}", i, extension);
        let part = load_image(&path).expect(&error);
        parts.push(part);
    }
}

pub fn count_parts(path: &str, extension: &str) -> usize {
    let dir = Path::new(path);
    let mut count = 0;

    if dir.is_dir() {
        for entry in fs::read_dir(dir).expect("Directory not found") {
            let entry = entry.expect("Error reading entry");
            let file_path = entry.path();

            if file_path.is_file() && file_path.extension().unwrap_or_default() == extension {
                count += 1;
            }
        }
    } else {
        println!("Provided path is not a directory");
    }

    count
}

// EXAMPLE 1
pub fn load_parts_1_1(parts: &mut Vec<RgbImage>, path: &str) {
    for i in 1..13 {
        let path = format!("{}part_{}.jpg", path, i);
        let error = format!("Error occurred while loading part_{}.jpg", i);
        let part = load_image(&path).expect(&error);
        parts.push(part);
    }
}

pub fn load_parts_1_2(parts: &mut Vec<RgbImage>, path: &str) {
    for i in 1..=12 {
        for j in 1..=29 {
            let path = format!("{}image{}x{}.png", path, i, j);
            let part = load_image(&path).expect("ERROR");
            parts.push(part);
        }
    }
}

// EXAMPLE 2
pub fn load_parts_2_1(parts: &mut Vec<RgbImage>, path: &str) {
    for i in 1..16 {
        let path = format!("{}part_{}.png", path, i);
        let error = format!("Error occurred while loading part_{}.png", i);
        let part = load_image(&path).expect(&error);
        parts.push(part);
    }
}

// EXAMPLE 3
pub fn load_parts_3(parts: &mut Vec<RgbImage>, path: &str) {
    for i in 0..191 {
        let path = format!("{}image{}.jpg", path, i);
        let error = format!("Error occurred while loading part_{}.png", i);
        let part = load_image(&path).expect(&error);
        parts.push(part);
    }
}

// EXAMPLE 4
pub fn load_parts_4(parts: &mut Vec<RgbImage>, path: &str) {
    for i in 0..14 {
        let path = format!("{}image{}.png", path, i);
        let error = format!("Error occurred while loading part_{}.png", i);
        let part = load_image(&path).expect(&error);
        parts.push(part);
    }
}

// EXAMPLE 5
pub fn load_parts_5(parts: &mut Vec<RgbImage>, path: &str) {
    for i in 0..95 {
        let path = format!("{}image{}.jpg", path, i);
        let error = format!("Error occurred while loading part_{}.png", i);
        let part = load_image(&path).expect(&error);
        parts.push(part);
    }
}

// EXAMPLE 6
pub fn load_parts_6_1(parts: &mut Vec<RgbImage>, path: &str) {
    for i in 0..=999 {
        let path = format!("{}image{}.png", path, i);
        let part = load_image(&path).expect("ERROR");
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

// Debug function
// pub fn print_matrix_portion(matrix: &Vec<Vec<(u8, u8, u8)>>, size: usize) {
//     for row in matrix.iter().take(size) {
//         for &pixel in row.iter().take(size) {
//             print!("{:?} ", pixel);
//         }
//         println!();
//     }
// }
