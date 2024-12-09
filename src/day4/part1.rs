use crate::common::{
    map::CharMap,
    math::{IVec2, UVec2},
    CARDINAL_INTERCARDINAL_DIRECTIONS,
};

pub fn solution(input: String) -> u32 {
    let map = CharMap::new(&input);

    let mut count = 0;
    let word = "XMAS";
    let search_string = word.chars().collect::<Vec<_>>();
    let search_string_len = search_string.len();

    for x in 0..map.width {
        for y in 0..map.height {
            let cur_pos = UVec2::new(x, y);
            let cur_char = map.get(cur_pos);

            if cur_char == search_string[0] {
                for (dir_x, dir_y) in CARDINAL_INTERCARDINAL_DIRECTIONS {
                    let dir = IVec2::new(dir_x, dir_y);
                    let mut is_match = true;
                    for (search_index, search_char) in
                        search_string[1..search_string_len].iter().enumerate().rev()
                    {
                        let search_index = (search_index + 1) as isize;
                        let new_pos = cur_pos + (dir * search_index);
                        if !new_pos.is_coord_safe(map.width, map.height)
                            || map.get_ivec_unchecked(new_pos) != *search_char
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
