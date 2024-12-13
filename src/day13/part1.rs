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

    let mut cheapest_winning_costs = 0;
    for machine in machines {
        let mut cheapest_win = u64::MAX;
        for b in 1_usize..101 {
            for a in 1_usize..101 {
                let destination = (machine.a * a) + (machine.b * b);
                if destination == machine.prize {
                    let cost = ((a * A_COST) + (b * B_COST)) as u64;
                    if cheapest_win > cost {
                        cheapest_win = cost;
                    }
                }
            }
        }
        if cheapest_win < u64::MAX {
            cheapest_winning_costs += cheapest_win;
        }
    }
    cheapest_winning_costs
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

    let mut ints = split2.map(|str| str.parse::<usize>().unwrap());

    UVec2::new(ints.next().unwrap(), ints.next().unwrap())
}
