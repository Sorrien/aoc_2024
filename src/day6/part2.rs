use crate::common::{
    cardinal_rot_right_90,
    math::{IVec2, UVec2},
    CARDINAL_DIRECTIONS,
};
use rayon::prelude::*;

pub const OBSTRUCTION: char = '#';
pub const MAX_STEPS: usize = 6000; //100000;

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

    let mut obstacle_success_counter = 0;

    for x in 0..width {
        for y in 0..height {
            if (x == start_x && y == start_y) || map[x][y] == OBSTRUCTION {
                continue;
            }

            let mut step_counter: usize = 0;

            let mut current_pos = UVec2::new(start_x, start_y);

            let mut is_in_bounds = true;
            let mut dir_index = 0;

            while is_in_bounds && step_counter < MAX_STEPS {
                let (dir_x, dir_y) = CARDINAL_DIRECTIONS[dir_index];
                let dir_vec = IVec2::new(dir_x, dir_y);
                let next_position = current_pos + dir_vec;

                if next_position.x < 0
                    || next_position.x >= width as isize
                    || next_position.y < 0
                    || next_position.y >= height as isize
                {
                    is_in_bounds = false;
                } else {
                    let next_char = map[next_position.x as usize][next_position.y as usize];
                    if next_char == OBSTRUCTION
                        || (next_position.x as usize == x && next_position.y as usize == y)
                    {
                        dir_index = cardinal_rot_right_90(dir_index);
                    } else {
                        current_pos = next_position.as_u_position();
                        step_counter += 1;
                    }
                }
            }

            if step_counter == MAX_STEPS {
                obstacle_success_counter += 1;
            }
        }
    }

    obstacle_success_counter
}

pub fn solution_parallel(input: String) -> u32 {
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

    (0..width)
        .into_par_iter()
        .map(|x| {
            (0..height)
                .into_par_iter()
                .filter_map(|y| {
                    if (x == start_x && y == start_y) || map[x][y] == OBSTRUCTION {
                        None
                    } else {
                        let mut step_counter: usize = 0;

                        let mut current_pos = UVec2::new(start_x, start_y);

                        let mut is_in_bounds = true;
                        let mut dir_index = 0;

                        while is_in_bounds && step_counter < MAX_STEPS {
                            let (dir_x, dir_y) = CARDINAL_DIRECTIONS[dir_index];
                            let dir_vec = IVec2::new(dir_x, dir_y);
                            let next_position = current_pos + dir_vec;

                            if next_position.x < 0
                                || next_position.x >= width as isize
                                || next_position.y < 0
                                || next_position.y >= height as isize
                            {
                                is_in_bounds = false;
                            } else {
                                let next_char =
                                    map[next_position.x as usize][next_position.y as usize];
                                if next_char == OBSTRUCTION
                                    || (next_position.x as usize == x
                                        && next_position.y as usize == y)
                                {
                                    dir_index = cardinal_rot_right_90(dir_index);
                                } else {
                                    current_pos = next_position.as_u_position();
                                    step_counter += 1;
                                }
                            }
                        }

                        if is_in_bounds {
                            Some(1)
                        } else {
                            None
                        }
                    }
                })
                .count() as u32
        })
        .sum()
}
