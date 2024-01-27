pub fn find_best_position(
    image_pix: &Vec<Vec<(u8, u8, u8)>>,
    part_pix: &Vec<Vec<(u8, u8, u8)>>,
) -> (usize, usize) {
    let parth = part_pix.len();
    let partw = part_pix[0].len();
    let mainh = image_pix.len();
    let mainw = image_pix[0].len();

    let mut best_score = f32::MAX;
    let mut best_position = (0, 0);

    for y in 0..=mainh - parth {
        for x in 0..=mainw - partw {
            let match_score = calculate_match_score(image_pix, part_pix, x, y);
            if match_score < best_score {
                best_score = match_score;
                best_position = (x, y);
            }
        }
    }

    (best_position.0, best_position.1)
}

fn calculate_match_score(
    main_matrix: &Vec<Vec<(u8, u8, u8)>>,
    part: &Vec<Vec<(u8, u8, u8)>>,
    start_x: usize,
    start_y: usize,
) -> f32 {
    let height = part.len();
    let width = part[0].len();
    let mut total_score = 0.0;

    for x in 0..width {
        total_score += euclidean_distance(main_matrix[start_y][start_x + x], part[0][x]);
        total_score += euclidean_distance(
            main_matrix[start_y + height - 1][start_x + x],
            part[height - 1][x],
        );
    }

    for y in 1..height - 1 {
        total_score += euclidean_distance(main_matrix[start_y + y][start_x], part[y][0]);
        total_score += euclidean_distance(
            main_matrix[start_y + y][start_x + width - 1],
            part[y][width - 1],
        );
    }

    total_score
}

fn euclidean_distance(pixel1: (u8, u8, u8), pixel2: (u8, u8, u8)) -> f32 {
    let r_diff = pixel1.0 as f32 - pixel2.0 as f32;
    let g_diff = pixel1.1 as f32 - pixel2.1 as f32;
    let b_diff = pixel1.2 as f32 - pixel2.2 as f32;
    (r_diff.powi(2) + g_diff.powi(2) + b_diff.powi(2)).sqrt()
}
