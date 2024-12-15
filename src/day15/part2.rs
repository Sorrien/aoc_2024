use crate::common::{map::CharMap, math::UVec2, CardinalDirection, CARDINAL_DIRECTIONS};

pub fn solution(input: String) -> u64 {
    let lines = input.lines();
    let mut map_lines = vec![];
    let mut instruction_lines = vec![];
    let mut is_after_break = false;
    for line in lines {
        if is_after_break {
            instruction_lines.push(line);
        } else {
            if line == "" {
                is_after_break = true;
            } else {
                map_lines.push(line);
            }
        }
    }

    let map_string = map_lines
        .iter()
        .map(|str| {
            let str = str
                .chars()
                .flat_map(|c| {
                    match c {
                        '#' | '.' => {
                            vec![c, c] //'s pizza
                        }
                        '@' => vec![c, '.'],
                        'O' => vec!['[', ']'],
                        _ => panic!("at the disco"),
                    }
                })
                .collect::<String>();

            format!("{}\n", str)
        })
        .collect::<String>();
    let mut map = CharMap::new(&map_string);

    let mut robot_pos = UVec2::ZERO;
    for x in 0..map.width {
        for y in 0..map.height {
            let pos = UVec2::new(x, y);
            if map.get(pos) == '@' {
                robot_pos = pos;
            }
        }
    }

    let instructions = instruction_lines
        .iter()
        .map(|line| {
            line.chars().map(|c| match c {
                '^' => CardinalDirection::North,
                '>' => CardinalDirection::East,
                'v' => CardinalDirection::South,
                '<' => CardinalDirection::West,
                _ => panic!("a new direction has been discovered!"),
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    for instruction in instructions {
        let mut copy_map = map.clone();
        if attempt_move(&mut copy_map, robot_pos, instruction) {
            let dir = CARDINAL_DIRECTIONS[instruction as usize];
            let desired_pos = (robot_pos + dir).as_u_position();
            copy_map.swap(robot_pos, desired_pos);
            robot_pos = desired_pos;
            map = copy_map;
        }
    }

    map.data
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter().enumerate().filter_map(
                move |(y, c)| {
                    if *c == '[' {
                        Some(100 * x + y)
                    } else {
                        None
                    }
                },
            )
        })
        .sum::<usize>() as u64
}

pub fn attempt_move(map: &mut CharMap, pos: UVec2, dir: CardinalDirection) -> bool {
    let dir_vec = CARDINAL_DIRECTIONS[dir as usize];
    let desired_pos = pos + dir_vec;

    let moved = if desired_pos.is_coord_safe(map.width, map.height) {
        let desired_pos = desired_pos.as_u_position();

        let current_in_desired_pos = map.get(desired_pos);

        match current_in_desired_pos {
            '#' => false,
            '[' | ']' => {
                let other_half_dir = if current_in_desired_pos == '[' {
                    CardinalDirection::East
                } else {
                    CardinalDirection::West
                };
                let other_half_dir_vec = CARDINAL_DIRECTIONS[other_half_dir as usize];

                let pos_2 = (desired_pos + other_half_dir_vec).as_u_position();

                let desired_pos_1 = desired_pos + dir_vec;
                let desired_pos_2 = pos_2 + dir_vec;

                if desired_pos_1.is_coord_safe(map.width, map.height)
                    && desired_pos_2.is_coord_safe(map.width, map.height)
                {
                    let desired_pos_1 = desired_pos_1.as_u_position();
                    let desired_pos_2 = desired_pos_2.as_u_position();

                    let moved = if other_half_dir == dir {
                        let moved_2 = attempt_move(map, pos_2, dir);
                        let moved = if moved_2 {
                            map.swap(pos_2, desired_pos_2);

                            let moved_1 = attempt_move(map, desired_pos, dir);
                            if !moved_1 {
                                map.swap(pos_2, desired_pos_2);
                            }

                            moved_1
                        } else {
                            false
                        };

                        if moved {
                            map.swap(desired_pos, desired_pos_1);
                        }

                        moved
                    } else {
                        let moved_1 = attempt_move(map, desired_pos, dir);
                        if moved_1 {
                            map.swap(desired_pos, desired_pos_1);
                        }

                        let moved = if moved_1 {
                            let moved_2 = attempt_move(map, pos_2, dir);
                            if !moved_2 {
                                map.swap(desired_pos, desired_pos_1);
                            }
                            moved_2
                        } else {
                            false
                        };

                        if moved {
                            map.swap(pos_2, desired_pos_2);
                        }

                        moved
                    };

                    moved
                } else {
                    false
                }
            }
            '.' => true,
            _ => panic!("undiscovered cell type! {}", current_in_desired_pos),
        }
    } else {
        false
    };

    moved
}
