use crate::common::{math::UVec2, CardinalDirection};

pub fn solution(input: String) -> u32 {
    let map = input
        .lines()
        .map(|str| str.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = map.len();
    let height = map[0].len();

    let (start_x, start_y) = map
        .iter()
        .enumerate()
        .filter_map(|(x, line)| {
            if let Some(y) = line.iter().position(|c| *c == '^') {
                Some((x, y))
            } else {
                None
            }
        })
        .next()
        .unwrap();

    let mut current_pos = UVec2::new(start_x, start_y);

    let mut is_in_bounds = true;
    let mut dir = CardinalDirection::North;
    let mut visited_positions = vec![current_pos];

    while is_in_bounds {
        let dir_vec = dir.get_x_y();
        let next_position = current_pos + dir_vec;

        if next_position.x < 0
            || next_position.x >= width as isize
            || next_position.y < 0
            || next_position.y >= height as isize
        {
            is_in_bounds = false;
        } else {
            let next_char = map[next_position.x as usize][next_position.y as usize];
            if next_char == '#' {
                dir = dir.rot_right_90();
            } else {
                current_pos = next_position.as_u_position();
                visited_positions.push(current_pos);
            }
        }
    }

    visited_positions.sort();
    visited_positions.dedup();

    visited_positions.len() as u32
}
