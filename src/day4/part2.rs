use crate::common::{
    math::{IVec2, UVec2},
    INTERCARDINAL_DIRECTIONS,
};

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
            let cur_pos = UVec2::new(x, y);

            if cur_char == 'A' {
                let chars = INTERCARDINAL_DIRECTIONS
                    .iter()
                    .filter_map(|(dir_x, dir_y)| {
                        let dir = IVec2::new(*dir_x, *dir_y);
                        let new_pos = cur_pos + dir;
                        if new_pos.is_coord_safe(width, height) {
                            let char = map[new_pos.x as usize][new_pos.y as usize];
                            Some(char)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                if chars.len() == 4 {
                    if is_x_mas(&chars[0..=1]) && is_x_mas(&chars[2..=3]) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn is_x_mas(chars: &[char]) -> bool {
    let first = chars[0];
    let second = chars[1];
    (first == 'M' && second == 'S') || (first == 'S' && second == 'M')
}
