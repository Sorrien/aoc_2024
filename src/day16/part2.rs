use std::collections::{HashMap, HashSet, VecDeque};

use crate::common::{
    cardinal_rot_left_90, cardinal_rot_right_90, map::CharMap, math::UVec2, CardinalDirection,
    CARDINAL_DIRECTIONS, ENUM_CARDINAL_DIRECTIONS,
};

const TURN_SCORE: usize = 1000;

pub fn solution(input: String) -> u64 {
    let map = CharMap::new(&input);

    let mut start_pos = UVec2::ZERO;
    for x in 0..map.width {
        for y in 0..map.height {
            let pos = UVec2::new(x, y);
            if map.get(pos) == 'S' {
                start_pos = pos;
                break;
            }
        }
    }

    let mut path_map = HashMap::<(UVec2, CardinalDirection), LinePath>::new();

    let mut visited = HashSet::new();

    let mut options_to_check = VecDeque::<(UVec2, CardinalDirection)>::new();
    for direction in ENUM_CARDINAL_DIRECTIONS {
        options_to_check.push_back((start_pos, direction));
    }

    while let Some((pos, cur_direction)) = options_to_check.pop_front() {
        let mut line_start_pos = pos;
        let mut cur_pos = pos;
        let cur_direction_vec = CARDINAL_DIRECTIONS[cur_direction as usize];
        let other_directions = [
            ENUM_CARDINAL_DIRECTIONS[cardinal_rot_right_90(cur_direction as usize)],
            ENUM_CARDINAL_DIRECTIONS[cardinal_rot_left_90(cur_direction as usize)],
        ];
        loop {
            visited.insert(cur_pos);
            let mut found_turn = false;
            for direction in &other_directions {
                let next_pos = (cur_pos + CARDINAL_DIRECTIONS[*direction as usize]).as_u_position();

                if !visited.contains(&next_pos) {
                    let next_char = map.get(next_pos);
                    if next_char == '.' {
                        options_to_check.push_back((cur_pos, *direction));
                        found_turn = true;
                    }
                }
            }

            if found_turn {
                path_map.insert(
                    (line_start_pos, cur_direction),
                    LinePath {
                        start: line_start_pos,
                        end: cur_pos,
                        direction: cur_direction,
                    },
                );
                line_start_pos = cur_pos;
            }

            let next_pos = (cur_pos + cur_direction_vec).as_u_position();

            let next_char = map.get(next_pos);
            if next_char == '.' {
                cur_pos = next_pos;
            } else if next_char == '#' {
                path_map.insert(
                    (line_start_pos, cur_direction),
                    LinePath {
                        start: line_start_pos,
                        end: cur_pos,
                        direction: cur_direction,
                    },
                );
                break;
            } else if next_char == 'E' {
                path_map.insert(
                    (line_start_pos, cur_direction),
                    LinePath {
                        start: line_start_pos,
                        end: next_pos,
                        direction: cur_direction,
                    },
                );
                break;
            }
        }
    }

    let path_map: HashMap<(UVec2, CardinalDirection), LinePath> =
        HashMap::from_iter(path_map.values().filter_map(|line_path| {
            if line_path.start == line_path.end {
                None
            } else {
                Some(((line_path.start, line_path.direction), *line_path))
            }
        }));

    let mut scores = Vec::<usize>::new();
    let mut path_options: VecDeque<(UVec2, CardinalDirection, usize, i32, Vec<LinePath>)> =
        VecDeque::<(UVec2, CardinalDirection, usize, i32, Vec<LinePath>)>::new();
    path_options.push_back((start_pos, CardinalDirection::East, 0, 0, vec![]));

    let mut followed_paths = Vec::<Vec<LinePath>>::new();

    while let Some((pos, direction, score, steps, paths)) = path_options.pop_front() {
        if let Some(min) = scores.iter().min() {
            if score > *min {
                continue;
            }
        }

        let steps = steps + 1;
        if steps > 500 {
            continue;
        }
        let pos = pos;
        let score: usize = score;
        let cur_direction = direction;
        let char = map.get(pos);
        if char == 'E' {
            followed_paths.push(paths);
            scores.push(score);
            continue;
        }
        if char == '#' {
            continue;
        } else {
            if let Some(line_path) = path_map.get(&(pos, cur_direction)) {
                if pos != line_path.end {
                    let len =
                        (line_path.end.as_i_position() - line_path.start.as_i_position()).length();

                    let mut paths = paths.clone();
                    paths.push(*line_path);
                    path_options.push_back((
                        line_path.end,
                        cur_direction,
                        score + len as usize,
                        steps,
                        paths,
                    ));
                }
            }

            let other_directions = [
                ENUM_CARDINAL_DIRECTIONS[cardinal_rot_right_90(cur_direction as usize)],
                ENUM_CARDINAL_DIRECTIONS[cardinal_rot_left_90(cur_direction as usize)],
            ];

            for direction in other_directions {
                if let Some(line_path) = path_map.get(&(pos, direction)) {
                    if pos != line_path.end {
                        let len = (line_path.end.as_i_position() - line_path.start.as_i_position())
                            .length();
                        let mut paths = paths.clone();
                        paths.push(*line_path);
                        path_options.push_back((
                            line_path.end,
                            direction,
                            score + len as usize + TURN_SCORE,
                            steps,
                            paths,
                        ));
                    }
                }
            }
        }
    }

    let best_score = *scores.iter().min().unwrap();

    let best_paths = followed_paths
        .iter()
        .enumerate()
        .filter_map(|(i, path)| {
            if scores[i] == best_score {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut cell_counter = 0;
    for x in 0..map.width {
        for y in 0..map.height {
            'best_paths_loop: for path in best_paths.iter() {
                for path in path.iter() {
                    let len = (path.end.as_i_position() - path.start.as_i_position()).length();
                    let pos = path.start;
                    for i in 0..=len {
                        let next_pos = (pos + (CARDINAL_DIRECTIONS[path.direction as usize] * i))
                            .as_u_position();
                        if next_pos.x == x && next_pos.y == y {
                            cell_counter += 1;
                            break 'best_paths_loop;
                        }
                    }
                }
            }
        }
    }

    cell_counter
}

#[derive(Clone, Copy, Debug)]
pub struct LinePath {
    start: UVec2,
    end: UVec2,
    direction: CardinalDirection,
}
