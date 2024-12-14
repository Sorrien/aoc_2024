use std::collections::HashSet;

use crate::common::math::{mod_floor, IVec2, UVec2};

pub fn solution(input: String, width: usize, height: usize) -> u64 {
    let robots = input
        .lines()
        .map(|line| {
            let mut values = line
                .split_ascii_whitespace()
                .map(|str| {
                    let mut split = str.split("=");
                    split.next().unwrap();
                    split.next().unwrap()
                })
                .map(|coord_string| coord_string.split(","))
                .flatten()
                .map(|str| str.parse::<isize>().unwrap());

            let pos_x = values.next().unwrap() as usize;
            let pos_y = values.next().unwrap() as usize;
            let vel_x = values.next().unwrap();
            let vel_y = values.next().unwrap();

            Robot {
                pos: UVec2::new(pos_x, pos_y),
                vel: IVec2::new(vel_x, vel_y),
            }
        })
        .collect::<Vec<_>>();

    let mut time: usize = 0;

    loop {
        let mut final_robot_positions = HashSet::<UVec2>::new();
        for robot in &robots {
            let distance_moved: IVec2 = robot.vel * time;
            let destination = robot.pos + distance_moved;

            if destination.is_coord_safe(height, width) {
                final_robot_positions.insert(destination.as_u_position());
            } else {
                let wrap_x = mod_floor(destination.x, width as isize);
                let wrap_y = mod_floor(destination.y, height as isize);

                final_robot_positions.insert(UVec2::new(wrap_x as usize, wrap_y as usize));
            }
        }

        let mut is_tree_border_detected = false;
        //christmas tree is 32x32 including the borders
        'y_loop: for y in 0..height - 31 {
            let mut row_length = 0;
            for x in 0..width - 27 {
                if final_robot_positions.contains(&UVec2::new(x, y)) {
                    row_length += 1;
                    if row_length > 5 {
                        is_tree_border_detected = true;
                        break 'y_loop;
                    }
                } else {
                    row_length = 0;
                }
            }
        }

        if is_tree_border_detected {
            /*             let mut map_string = String::new();

            for y in 0..height {
            for x in 0..width {
                    let mut robot_count = 0;
                    for pos in final_robot_positions.iter() {
                        if pos.x == x && pos.y == y {
                            robot_count += 1;
                        }
                    }

                    let string = if robot_count > 0 {
                        String::from("#")
                    } else {
                        String::from("_")
                    };
                    map_string.push_str(&string);
                }
                map_string.push_str("\n");
            }
            eprintln!("{map_string}"); */
            break;
        }

        time += 1;
    }

    time as u64
}

#[derive(Debug)]
pub struct Robot {
    pos: UVec2,
    vel: IVec2,
}
