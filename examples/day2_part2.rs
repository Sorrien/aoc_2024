use aoc_2024::day2::part2::solution;
use std::fs;

fn main() {
    let input_string = fs::read_to_string("inputs/day2.txt").expect("failed to load input file");

    let sum = solution(input_string);

    println!("{}", sum);
}
