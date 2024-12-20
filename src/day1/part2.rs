use std::vec;

pub fn solution(input: String) -> u32 {
    let mut left_list = vec![];
    let mut right_list = vec![];
    let lines = input.lines();
    for line in lines {
        let split = line.split_ascii_whitespace();

        let mut iter = split.map(|str| str.parse::<u32>().unwrap());
        let left = iter.next().unwrap();
        let right = iter.next().unwrap();

        left_list.push(left);
        right_list.push(right);
    }

    right_list.sort();

    let sum = left_list
        .iter()
        .map(|left| {
            if let Some(position) = right_list.iter().position(|x| *x == *left) {
                let mut score = 1;
                let mut position: usize = position + 1;
                loop {
                    if right_list[position] == *left {
                        score += 1;
                        position += 1;
                    } else {
                        break;
                    }
                }
                left * score
            } else {
                0
            }
        })
        .sum();

    sum
}
