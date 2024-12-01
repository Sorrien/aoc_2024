use aoc_2024::day1::part1::solution;
use std::fs;

fn main() {
    let input_string = fs::read_to_string("inputs/day1.txt").expect("failed to load input file");

    let sum = solution(input_string);

    println!("{}", sum);
}
