mod common;
use aoc_2024::day8::part2::solution;
use common::*;
use std::fs;

fn main() {
    run_solution(false, || {
        let input_string =
            fs::read_to_string("inputs/day8.txt").expect("failed to load input file");

        let sum = solution(input_string);
        sum.to_string()
    });
}
