pub fn solution(input: String) -> u64 {
    let all_operator_combos = (1..15)
        .map(|i| get_all_possible_operator_combos(i))
        .collect::<Vec<_>>();

    input
        .lines()
        .filter_map(|line| {
            let mut first_split = line.split(":");
            let test_val = first_split.next().unwrap().parse::<u64>().unwrap();
            let values = first_split
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|str| str.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            let operator_count = values.len() - 1;
            let mut is_pass = false;

            let operator_combos = &all_operator_combos[operator_count - 1];

            for operators in operator_combos {
                let mut sum = values[0];
                for i in 0..operator_count {
                    let b = values[i + 1];
                    let op = operators[i];

                    match op {
                        Operator::Add => sum += b,
                        Operator::Mul => sum *= b,
                    };
                }

                if sum == test_val {
                    is_pass = true;
                    break;
                }
            }

            if is_pass {
                Some(test_val)
            } else {
                None
            }
        })
        .sum()
}

pub fn get_all_possible_operator_combos(len: usize) -> Vec<Vec<Operator>> {
    let len = len;
    let mut results = vec![];

    for operator in OPERATORS {
        let mut result = vec![];
        result.push(operator);
        results.push(result);
    }

    let results = permute(results, 1, len);

    results
}

pub fn permute(combos: Vec<Vec<Operator>>, index: usize, len: usize) -> Vec<Vec<Operator>> {
    let mut results = vec![];
    if index == len {
        results = combos.clone();
        return results;
    }
    for combo in combos {
        for operator in OPERATORS {
            let mut result = combo.clone();
            result.push(operator);
            results.push(result);
        }
    }

    permute(results, index + 1, len)
}

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    Add = 0,
    Mul = 1,
}

pub const OPERATORS: [Operator; 2] = [Operator::Add, Operator::Mul];
