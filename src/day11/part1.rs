use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn solution(input: String) -> u64 {
    day11(input, 25)
}

/* pub fn day11(input: String, iterations: usize) -> u64 {
    let mut stones = input
        .split_ascii_whitespace()
        .map(|str| str.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut new_stones = vec![];
    for _ in 0..iterations {
        for stone in stones {
            if stone == 0 {
                new_stones.push(1);
            } else {
                let stone_str = stone.to_string();
                if stone_str.len() % 2 == 0 {
                    let (left, right) = stone_str.split_at(stone_str.len() / 2);
                    let left_right = [left, right];
                    let parsed = left_right.iter().map(|str| str.parse::<u64>().unwrap());
                    new_stones.extend(parsed);
                } else {
                    new_stones.push(stone * 2024);
                }
            }
        }
        stones = new_stones.clone();
        new_stones.clear();
    }
    stones.len() as u64
} */

pub fn day11(input: String, iterations: usize) -> u64 {
    let mut stones = input
        .split_ascii_whitespace()
        .map(|str| str.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..iterations {
        stones = stones
            .par_iter()
            .map(|stone| {
                if *stone == 0 {
                    vec![1]
                } else {
                    let stone_str = stone.to_string();
                    if stone_str.len() % 2 == 0 {
                        let (left, right) = stone_str.split_at(stone_str.len() / 2);
                        let left_right = [left, right];
                        let parsed = left_right.iter().map(|str| str.parse::<u64>().unwrap());
                        parsed.collect::<Vec<_>>()
                    } else {
                        vec![stone * 2024]
                    }
                }
            })
            .flatten()
            .collect::<Vec<_>>();
    }
    stones.len() as u64
}
