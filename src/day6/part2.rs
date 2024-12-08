use crate::common::math::{IVec2, UVec2};
use rayon::prelude::*;

pub const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];
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
            let mut dir = Direction::North;

            while is_in_bounds && step_counter < MAX_STEPS {
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
                    if next_char == OBSTRUCTION
                        || (next_position.x as usize == x && next_position.y as usize == y)
                    {
                        dir = dir.rot_right_90();
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
                        let mut dir = Direction::North;

                        while is_in_bounds && step_counter < MAX_STEPS {
                            let dir_vec = dir.get_x_y();
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
                                    dir = dir.rot_right_90();
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

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}

impl Direction {
    pub fn get_x_y(&self) -> IVec2 {
        let (x, y) = DIRECTIONS[*self as usize];
        IVec2::new(x, y)
    }
    pub fn rot_right_90(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}
