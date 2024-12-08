use crate::common::math::UVec2;

pub fn solution(input: String) -> u64 {
    let map = input
        .lines()
        .map(|str| str.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = map.len();
    let height = map[0].len();

    let mut antinode_positions = vec![];

    for x in 0..width {
        for y in 0..height {
            let cur_char = map[x][y];
            let cur_pos = UVec2::new(x, y);

            if cur_char != '.' {
                for next_x in 0..width {
                    for next_y in 0..height {
                        if x != next_x || y != next_y {
                            let next_pos = UVec2::new(next_x, next_y);
                            let next_char = map[next_pos.x][next_pos.y];

                            if next_char == cur_char {
                                let distance = next_pos.as_i_position() - cur_pos.as_i_position();
                                //let direction = distance.normalize();

                                //println!("distance: {:?} direction: {:?}", distance, direction);

                                let antinode_1_pos = cur_pos + (distance * -1);
                                let antinode_2_pos = next_pos + distance;

                                if antinode_1_pos.is_coord_safe(width, height) {
                                    antinode_positions.push(antinode_1_pos);
                                }
                                if antinode_2_pos.is_coord_safe(width, height) {
                                    antinode_positions.push(antinode_2_pos);
                                }

                                let mut index = 0;
                                let mut is_in_boundary = true;

                                while is_in_boundary {
                                    let next_in_line_pos = cur_pos + ((distance * -1) * index);
                                    if next_in_line_pos.is_coord_safe(width, height) {
                                        antinode_positions.push(next_in_line_pos);
                                        index += 1;
                                    } else {
                                        is_in_boundary = false;
                                    }
                                }

                                let mut index = 0;
                                let mut is_in_boundary = true;

                                while is_in_boundary {
                                    let next_in_line_pos = next_pos + (distance * index);
                                    if next_in_line_pos.is_coord_safe(width, height) {
                                        antinode_positions.push(next_in_line_pos);
                                        index += 1;
                                    } else {
                                        is_in_boundary = false;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    antinode_positions.sort();
    antinode_positions.dedup();

    println!("{:?}", antinode_positions);

    antinode_positions.len() as u64
}

//4,7 is wrong
//6,8 is wrong
//8,9 is wrong