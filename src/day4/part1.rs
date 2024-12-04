pub const SEARCH_DIRS: [(isize, isize); 8] = [
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

    for x in 0..width {
        for y in 0..height {
            let cur_char = map[x][y];

            if cur_char == 'X' {
                for (dir_x, dir_y) in SEARCH_DIRS {
                    let (x, y) = apply_dir(x as isize, y as isize, dir_x, dir_y);
                    if is_coord_safe(x, y, width, height) {
                        if map[x as usize][y as usize] == 'M' {
                            let (x, y) = apply_dir(x, y, dir_x, dir_y);
                            if is_coord_safe(x, y, width, height) {
                                if map[x as usize][y as usize] == 'A' {
                                    let (x, y) = apply_dir(x, y, dir_x, dir_y);
                                    if is_coord_safe(x, y, width, height) {
                                        if map[x as usize][y as usize] == 'S' {
                                            count += 1;
                                        }
                                    }
                                }
                            }
                        }
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
    ((x as isize - dir_x), (y as isize - dir_y))
}
