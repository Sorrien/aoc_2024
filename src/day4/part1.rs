use crate::common::{
    math::{IVec2, UVec2},
    CARDINAL_INTERCARDINAL_DIRECTIONS,
};

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
            let cur_pos = UVec2::new(x, y);

            if cur_char == search_string[0] {
                for (dir_x, dir_y) in CARDINAL_INTERCARDINAL_DIRECTIONS {
                    let dir = IVec2::new(dir_x, dir_y);
                    let mut is_match = true;
                    for (search_index, search_char) in
                        search_string[1..search_string_len].iter().enumerate().rev()
                    {
                        let search_index = (search_index + 1) as isize;
                        let new_pos = cur_pos + (dir * search_index);
                        if !new_pos.is_coord_safe(width, height)
                            || map[new_pos.x as usize][new_pos.y as usize] != *search_char
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
