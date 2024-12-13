use crate::common::math::UVec2;

const A_COST: usize = 3;
const B_COST: usize = 1;

pub fn solution(input: String) -> u64 {
    let mut machines = vec![];

    let mut lines = input.lines();

    let mut is_finished = false;
    while !is_finished {
        for _ in 0..4 {
            let button_a_str = lines.next().unwrap();
            let button_b_str = lines.next().unwrap();
            let prize_str = lines.next().unwrap();

            let a = button(button_a_str);
            let b = button(button_b_str);
            let prize = prize(prize_str);

            machines.push(Machine { a, b, prize });

            is_finished = !lines.next().is_some();
        }
    }

    machines
        .iter()
        .map(|machine| {
            let (x, y) = solve_lin_equation_float(
                machine.a.x as f64,
                machine.b.x as f64,
                machine.a.y as f64,
                machine.b.y as f64,
                machine.prize.x as f64,
                machine.prize.y as f64,
            );
            let x = x.round() as usize;
            let y = y.round() as usize;

            let destination = UVec2::new(
                machine.a.x * x + machine.b.x * y,
                machine.a.y * x + machine.b.y * y,
            );

            (if destination == machine.prize {
                (x * A_COST) + (y * B_COST)
            } else {
                0
            }) as u64
        })
        .sum()
}

pub struct Machine {
    a: UVec2,
    b: UVec2,
    prize: UVec2,
}

pub fn button(input: &str) -> UVec2 {
    let mut split1 = input.split("X+");
    split1.next().unwrap();
    let split2 = split1.next().unwrap().split(", Y+");

    let mut ints = split2.map(|str| str.parse::<usize>().unwrap());

    UVec2::new(ints.next().unwrap(), ints.next().unwrap())
}

pub fn prize(input: &str) -> UVec2 {
    let mut split1 = input.split("X=");
    split1.next().unwrap();
    let split2 = split1.next().unwrap().split(", Y=");

    let mut ints = split2.map(|str| str.parse::<usize>().unwrap() + 10000000000000);

    UVec2::new(ints.next().unwrap(), ints.next().unwrap())
}

fn solve_lin_equation_float(a: f64, b: f64, c: f64, d: f64, u: f64, v: f64) -> (f64, f64) {
    let f = u * c / a;
    let g = b * c / a;
    let y = (v - f) / (d - g);
    return ((f - g * y) / c, y);
}
