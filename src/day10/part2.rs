use crate::common::{map::IntMap, math::UVec2, CARDINAL_DIRECTIONS};

pub fn solution(input: String) -> u64 {
    let map = IntMap::new(&input);

    (0..map.width)
        .map(|x| {
            (0..map.height)
                .map(|y| {
                    let pos = UVec2::new(x, y);

                    let mut local_sum = 0;
                    let val = map.get(pos);
                    if val == 0 {
                        let mut is_finished = false;
                        let mut current_positions = vec![pos];

                        while !is_finished {
                            let mut next_positions = vec![];
                            for position in &current_positions {
                                let cur_val = map.get(*position);
                                for direction in CARDINAL_DIRECTIONS {
                                    let next_pos = *position + direction;

                                    if let Some(next_val) = map.get_ivec_checked(next_pos) {
                                        if cur_val + 1 == next_val {
                                            if next_val == 9 {
                                                local_sum += 1;
                                            } else {
                                                next_positions.push(next_pos.as_u_position());
                                            }
                                        }
                                    }
                                }
                            }
                            if next_positions.len() == 0 {
                                is_finished = true;
                            } else {
                                current_positions = next_positions;
                            }
                        }
                    }
                    local_sum
                })
                .sum::<u64>()
        })
        .sum()
}
