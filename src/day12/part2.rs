use std::collections::{HashSet, VecDeque};

use crate::common::{
    map::CharMap, math::UVec2, CardinalDirection, CARDINAL_DIRECTIONS, ENUM_CARDINAL_DIRECTIONS,
    ENUM_EAST_WEST_DIRS, ENUM_NORTH_SOUTH_DIRS,
};

pub fn solution(input: String) -> u64 {
    let map = CharMap::new(&input);
    let mut visited: HashSet<UVec2> = HashSet::new();
    let mut result: u32 = 0;
    for (x, row) in map.data.iter().enumerate() {
        for (y, char) in row.iter().enumerate() {
            let pos = UVec2::new(x, y);
            if visited.contains(&pos) {
                continue;
            }

            let mut current_region: HashSet<UVec2> = HashSet::new();
            visited.insert(pos);
            current_region.insert(pos);

            let mut stack: VecDeque<UVec2> = VecDeque::new();
            stack.push_back(pos);

            while stack.len() > 0 {
                let current_plot = stack.pop_front().unwrap();

                for direction in CARDINAL_DIRECTIONS {
                    let new_pos = current_plot + direction;

                    if new_pos.is_coord_safe(map.width, map.height) {
                        let new_pos = new_pos.as_u_position();
                        if map.get(new_pos) == *char && !visited.contains(&new_pos) {
                            current_region.insert(new_pos);
                            visited.insert(new_pos);
                            stack.push_back(new_pos);
                        }
                    }
                }
            }
            let mut sides: HashSet<(UVec2, CardinalDirection)> = HashSet::new();
            for slot in current_region.iter() {
                for wall_dir in ENUM_CARDINAL_DIRECTIONS {
                    let wall_dir_vec = wall_dir.get_direction();
                    if !current_region.contains(&(*slot + wall_dir_vec).as_u_position()) {
                        let mut is_side_found = false;

                        let check_dirs = match wall_dir {
                            CardinalDirection::North | CardinalDirection::South => {
                                ENUM_EAST_WEST_DIRS
                            }
                            CardinalDirection::West | CardinalDirection::East => {
                                ENUM_NORTH_SOUTH_DIRS
                            }
                        };
                        for dir in check_dirs {
                            let dir_vec = dir.get_direction();
                            let mut next_pos = *slot + dir_vec;

                            if next_pos.is_coord_safe(map.width, map.height) {
                                loop {
                                    let new_pos = next_pos.as_u_position();
                                    if !current_region.contains(&new_pos)
                                        || (current_region.contains(&new_pos)
                                            && current_region.contains(
                                                &(new_pos + wall_dir_vec).as_u_position(),
                                            ))
                                    {
                                        break;
                                    } else if sides.contains(&(new_pos, wall_dir)) {
                                        is_side_found = true;
                                        break;
                                    } else {
                                        next_pos = next_pos + dir_vec;
                                        if !next_pos.is_coord_safe(map.width, map.height) {
                                            break;
                                        }
                                    }
                                }
                            }
                        }

                        if !is_side_found {
                            sides.insert((*slot, wall_dir));
                        }
                    }
                }
            }

            result += (current_region.len() * sides.len()) as u32;
        }
    }
    result as u64
}
