mod common;
use aoc_2024::day2::part1::solution;
use common::*;
use std::fs;

fn main() {
    run_solution(true, || {
        let input_string =
            fs::read_to_string("inputs/day2.txt").expect("failed to load input file");

        let sum = solution(input_string);
        sum.to_string()
    });
}
