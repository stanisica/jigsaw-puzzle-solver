pub fn find_best_position(
    image_pix: &Vec<Vec<(u8, u8, u8)>>,
    part_pix: &Vec<Vec<(u8, u8, u8)>>,
) -> (usize, usize) {
    let parth = part_pix.len();
    let partw = part_pix[0].len();
    let mainh = image_pix.len();
    let mainw = image_pix[0].len();

    let mut best_score = 0;
    let mut best_position = (0, 0);

    for y in 0..mainh - parth + 1 {
        for x in 0..mainw - partw + 1 {
            let window = create_search_window(&image_pix, x, y, partw, parth);
            let match_score = calculate_match_score(&window, &part_pix);
            if match_score > best_score {
                best_score = match_score;
                best_position = (x, y);
            }
        }
    }

    (best_position.0, best_position.1)
}

fn calculate_match_score(window: &Vec<Vec<(u8, u8, u8)>>, part: &Vec<Vec<(u8, u8, u8)>>) -> usize {
    let mut match_score = 0;
    let height = part.len();
    let width = part[0].len();

    for x in 0..width {
        if window[0][x] == part[0][x] {
            match_score += 1;
        }
        if window[height - 1][x] == part[height - 1][x] {
            match_score += 1;
        }
    }

    for y in 1..height - 1 {
        if window[y][0] == part[y][0] {
            match_score += 1;
        }
        if window[y][width - 1] == part[y][width - 1] {
            match_score += 1;
        }
    }

    match_score
}

pub fn create_search_window(
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
