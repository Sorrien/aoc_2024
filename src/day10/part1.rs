use crate::common::{
    map::IntMap,
    math::{IVec2, UVec2},
    CARDINAL_DIRECTIONS,
};

pub fn solution(input: String) -> u64 {
    let map = IntMap::new(&input);

    let cardinal_directions = CARDINAL_DIRECTIONS
        .iter()
        .map(|(x, y)| IVec2::new(*x, *y))
        .collect::<Vec<_>>();

    let mut global_sum = 0;
    for x in 0..map.width {
        for y in 0..map.height {
            let pos = UVec2::new(x, y);

            let val = map.get(pos);
            if val == 0 {
                let mut trail_end_positions = vec![];
                let mut is_finished = false;
                let mut current_positions = vec![pos];

                while !is_finished {
                    let mut next_positions = vec![];
                    for position in &current_positions {
                        let cur_val = map.get(*position);
                        for direction in &cardinal_directions {
                            let next_pos = *position + *direction;

                            if let Some(next_val) = map.get_ivec_checked(next_pos) {
                                if cur_val + 1 == next_val {
                                    if next_val == 9 {
                                        trail_end_positions.push(next_pos.as_u_position());
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

                trail_end_positions.sort();
                trail_end_positions.dedup();
                global_sum += trail_end_positions.len() as u64
            }
        }
    }

    global_sum
}
