pub const SEARCH_DIRS: [(isize, isize); 4] = [(-1, 1), (1, -1), (1, 1), (-1, -1)];

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

            if cur_char == 'A' {
                let chars = SEARCH_DIRS
                    .iter()
                    .filter_map(|(dir_x, dir_y)| {
                        let (x, y) = apply_dir(x as isize, y as isize, *dir_x, *dir_y);
                        if is_coord_safe(x, y, width, height) {
                            let char = map[x as usize][y as usize];
                            Some(char)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                if chars.len() == 4 {
                    let first = chars[0];
                    let second = chars[1];
                    let third = chars[2];
                    let fourth = chars[3];

                    if ((first == 'M' && second == 'S') || (first == 'S' && second == 'M'))
                        && ((third == 'M' && fourth == 'S') || (third == 'S' && fourth == 'M'))
                    {
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
    ((x as isize - dir_x), (y as isize - dir_y))
}
