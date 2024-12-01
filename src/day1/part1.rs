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
    //let x = lines.map(|line| line.split_ascii_whitespace()).map(|split| split.map(|str| str.parse::<u32>().unwrap()));

    left_list.sort();
    right_list.sort();

    let sum = left_list
        .iter()
        .zip(right_list)
        .map(|(l, r)| l.abs_diff(r))
        .sum();

    sum
}
