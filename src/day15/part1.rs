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
        .map(|str| format!("{}\n", str))
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
        if attempt_move(&mut map, robot_pos, instruction) {
            let dir = CARDINAL_DIRECTIONS[instruction as usize];
            let desired_pos = (robot_pos + dir).as_u_position();
            robot_pos = desired_pos;
        }

        let debug_map_string = map
            .data
            .iter()
            .map(|row| format!("{}\n", String::from_iter(row)))
            .collect::<String>();

        eprintln!("{debug_map_string}");
    }

    /*     let debug_map_string = map
        .data
        .iter()
        .map(|row| format!("{}\n", String::from_iter(row)))
        .collect::<String>();

    eprintln!("{debug_map_string}"); */

    map.data
        .iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter().enumerate().filter_map(move |(y, c)| {
                if *c == 'O' {
                    Some((100 * (x)) + (y))
                } else {
                    None
                }
            })
        })
        .sum::<usize>() as u64
}

pub fn attempt_move(map: &mut CharMap, pos: UVec2, dir: CardinalDirection) -> bool {
    let dir_vec = CARDINAL_DIRECTIONS[dir as usize];
    let desired_pos = (pos + dir_vec).as_u_position();
    let current_in_desired_pos = map.get(desired_pos);

    match current_in_desired_pos {
        '#' => false, //is wall, do nothing
        'O' => {
            //is box, attempt push
            let moved = attempt_move(map, desired_pos, dir);
            if moved {
                map.swap(pos, desired_pos); //is empty. move.
            }
            moved
        }
        '.' => {
            map.swap(pos, desired_pos); //is empty. move.
            true
        }
        '@' => {
            println!("{:?}", map.data);
            panic!(
                "how did we get here? pos:{:?} desired_pos: {:?}",
                pos, desired_pos
            )
        }
        _ => panic!("undiscovered cell type! {}", current_in_desired_pos),
    }
}
