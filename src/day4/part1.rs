const SEARCH_DIRS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

pub fn solution(input: String) -> u32 {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = map.len();
    let height = map[0].len();
    let mut count = 0;
    let word = "XMAS";
    let search_string = word.chars().collect::<Vec<_>>();
    let search_string_len = search_string.len();

    for x in 0..width {
        for y in 0..height {
            let cur_char = map[x][y];

            if cur_char == search_string[0] {
                for (dir_x, dir_y) in SEARCH_DIRS {
                    let mut is_match = true;
                    for (search_index, search_char) in
                        search_string[1..search_string_len].iter().enumerate().rev()
                    {
                        let search_index = (search_index + 1) as isize;
                        let (x, y) = apply_dir(
                            x as isize,
                            y as isize,
                            dir_x * search_index as isize,
                            dir_y * search_index as isize,
                        );
                        if !is_coord_safe(x, y, width, height)
                            || map[x as usize][y as usize] != *search_char
                        {
                            is_match = false;
                            break;
                        }
                    }
                    if is_match {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn is_coord_safe(x: isize, y: isize, width: usize, height: usize) -> bool {
    x >= 0 && x < width as isize && y >= 0 && y < height as isize
}

fn apply_dir(x: isize, y: isize, dir_x: isize, dir_y: isize) -> (isize, isize) {
    ((x + dir_x), (y + dir_y))
}
