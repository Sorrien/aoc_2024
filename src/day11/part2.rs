use std::collections::HashMap;

pub fn solution(input: String) -> u64 {
    let stones = input
        .split_ascii_whitespace()
        .map(|str| str.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut stone_map = HashMap::<u64, u64>::new();
    for stone in stones.iter() {
        stone_map.insert(*stone, 1);
    }

    for _ in 0..75 {
        let mut new_stones = HashMap::<u64, u64>::new();
        for (stone, count) in stone_map.iter() {
            if *stone == 0 {
                if let Some(n) = new_stones.get_mut(&1) {
                    *n += count;
                } else {
                    new_stones.insert(1, *count);
                }
            } else {
                let stone_str = stone.to_string();
                if stone_str.len() % 2 == 0 {
                    let (left, right) = stone_str.split_at(stone_str.len() / 2);
                    let left_right = [left, right];
                    let parsed = left_right.iter().map(|str| str.parse::<u64>().unwrap());
                    let items = parsed.collect::<Vec<_>>();

                    for key in items {
                        if let Some(n) = new_stones.get_mut(&key) {
                            *n += count;
                        } else {
                            new_stones.insert(key, *count);
                        }
                    }
                } else {
                    let key = stone * 2024;
                    if let Some(n) = new_stones.get_mut(&key) {
                        *n += count;
                    } else {
                        new_stones.insert(key, *count);
                    }
                }
            }
        }
        stone_map = new_stones;
    }

    stone_map.values().sum()
}
