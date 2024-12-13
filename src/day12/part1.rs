use std::collections::HashMap;

use crate::common::{map::CharMap, math::UVec2, CARDINAL_DIRECTIONS};

pub fn solution(input: String) -> u64 {
    let map = CharMap::new(&input);

    let mut regions: Vec<Vec<_>> = vec![];
    let mut pos_to_region_hashmap = HashMap::<UVec2, usize>::new();
    for x in 0..map.width {
        for y in 0..map.height {
            let pos = UVec2::new(x, y);
            let cur_char = map.get(pos);
            for direction in CARDINAL_DIRECTIONS {
                let new_pos = pos + direction;

                if let Some(char) = map.get_ivec_checked(new_pos) {
                    let new_pos = new_pos.as_u_position();

                    if char == cur_char {
                        if pos_to_region_hashmap.contains_key(&pos) {
                            let pos_region_index = pos_to_region_hashmap[&pos];

                            if pos_to_region_hashmap.contains_key(&new_pos) {
                                let new_pos_region_index = pos_to_region_hashmap[&new_pos];
                                if pos_region_index != new_pos_region_index {
                                    let mut region_a = regions[pos_region_index].clone();
                                    let region_b = regions[new_pos_region_index].clone();
                                    region_a.extend(region_b);
                                    region_a.sort();
                                    region_a.dedup();
                                    regions[pos_region_index] = region_a;
                                    regions[new_pos_region_index] = vec![];

                                    for vec in &regions[pos_region_index] {
                                        pos_to_region_hashmap.insert(*vec, pos_region_index);
                                    }
                                }
                            } else {
                                regions[pos_region_index].push(new_pos);
                                pos_to_region_hashmap.insert(new_pos, pos_region_index);
                            }
                        } else {
                            if pos_to_region_hashmap.contains_key(&new_pos) {
                                let new_pos_region_index = pos_to_region_hashmap[&new_pos];
                                regions[new_pos_region_index].push(pos);
                                pos_to_region_hashmap.insert(pos, new_pos_region_index);
                            } else {
                                let region = vec![pos, new_pos];
                                regions.push(region);
                                let new_region_index = regions.len() - 1;
                                pos_to_region_hashmap.insert(pos, new_region_index);
                                pos_to_region_hashmap.insert(new_pos, new_region_index);
                            }
                        }
                    }
                }
            }

            if !pos_to_region_hashmap.contains_key(&pos) {
                let region = vec![pos];
                regions.push(region);
                let new_region_index = regions.len() - 1;
                pos_to_region_hashmap.insert(pos, new_region_index);
            }
        }
    }

    let regions = regions
        .iter()
        .filter_map(|region| {
            if region.len() > 0 {
                Some(region.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    /*     println!("regions: {}", regions.len());

    for region in &regions {
        let mut test_map = vec![];
        for x in 0..map.width {
            let mut row = vec![];
            for y in 0..map.height {
                let mut is_in_region = false;
                for pos in region {
                    if pos.x == x && pos.y == y {
                        is_in_region = true;
                        break;
                    }
                }

                let char = if is_in_region {
                    map.get(UVec2::new(x, y))
                } else {
                    '_'
                };
                row.push(char);
            }
            test_map.push(row);
        }

        let string = test_map
            .iter()
            .map(|row| format!("{}\n", row.iter().collect::<String>()))
            .collect::<String>();
        println!("{}", string);
        println!("");
    } */

    let mut sum = 0;
    for region in &regions {
        let area = region.len();

        let mut region_hashmap = HashMap::<UVec2, bool>::new();
        for pos in region {
            region_hashmap.insert(*pos, true);
        }

        let mut perimeter = 0;

        for pos in region {
            for direction in CARDINAL_DIRECTIONS {
                let adj_pos = *pos + direction;
                if !adj_pos.is_coord_safe(map.width, map.height)
                    || !region_hashmap.contains_key(&adj_pos.as_u_position())
                {
                    perimeter += 1;
                }
            }
        }

        sum += area * perimeter;
    }

    /*
    let mut alphabet_region_counter = [0; 26];
    for (i, char) in ALPHABET.iter().enumerate() {
        for region in &regions {
            let c = map.get(region[0]);
            if c == *char {
                alphabet_region_counter[i] += 1;
            }
        }
    }
    for (i, count) in alphabet_region_counter.iter().enumerate() {
        if *count > 0 {
            println!("{}: {}", ALPHABET[i], count);
        }
    } */

    sum as u64
}
