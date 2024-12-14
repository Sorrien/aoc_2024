use crate::common::math::{mod_floor, IVec2, UVec2};

const TIME: usize = 100;

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

    let mut quad_counts = vec![0, 0, 0, 0];

    let half_width = (width as f64 / 2.0).round() as usize - 1;
    let half_height = (height as f64 / 2.0).round() as usize - 1;

    for robot in &robots {
        let distance_moved = robot.vel * TIME;
        let destination = robot.pos + distance_moved;

        let pos = if destination.is_coord_safe(width, height) {
            destination.as_u_position()
        } else {
            let wrap_x = mod_floor(destination.x, width as isize);
            let wrap_y = mod_floor(destination.y, height as isize);

            UVec2::new(wrap_x as usize, wrap_y as usize)
        };

        if pos.x < half_width && pos.y < half_height {
            quad_counts[0] += 1;
        } else if pos.x > half_width && pos.y < half_height {
            quad_counts[1] += 1;
        } else if pos.x < half_width && pos.y > half_height {
            quad_counts[2] += 1;
        } else if pos.x > half_width && pos.y > half_height {
            quad_counts[3] += 1;
        }
    }

    let mut result = quad_counts[0];

    for i in 1..quad_counts.len() {
        result *= quad_counts[i];
    }
    result
}

#[derive(Debug)]
pub struct Robot {
    pos: UVec2,
    vel: IVec2,
}
