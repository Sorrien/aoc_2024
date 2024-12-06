pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day1;
    use crate::day2;
    use crate::day3;
    use crate::day4;
    use crate::day5;
    use crate::day6;

    #[test]
    fn day1_part1() {
        let input_string =
            fs::read_to_string("inputs/day1.txt").expect("failed to load input file");

        let result = day1::part1::solution(input_string);
        assert_eq!(result, 1882714);
    }

    #[test]
    fn day1_part2() {
        let input_string =
            fs::read_to_string("inputs/day1.txt").expect("failed to load input file");

        let result = day1::part2::solution(input_string);
        assert_eq!(result, 19437052);
    }

    #[test]
    fn day2_part1_ex() {
        let input_string =
            fs::read_to_string("inputs/day2_example.txt").expect("failed to load input file");

        let result = day2::part1::solution(input_string);
        assert_eq!(result, 2);
    }

    #[test]
    fn day2_part1() {
        let input_string =
            fs::read_to_string("inputs/day2.txt").expect("failed to load input file");

        let result = day2::part1::solution(input_string);
        assert_eq!(result, 502);
    }

    #[test]
    fn day2_part2_ex() {
        let input_string =
            fs::read_to_string("inputs/day2_example.txt").expect("failed to load input file");

        let result = day2::part2::solution(input_string);
        assert_eq!(result, 4);
    }

    #[test]
    fn day2_part2() {
        let input_string =
            fs::read_to_string("inputs/day2.txt").expect("failed to load input file");

        let result = day2::part2::solution(input_string);
        assert_eq!(result, 544);
    }

    #[test]
    fn day3_part1_ex() {
        let input_string =
            fs::read_to_string("inputs/day3_example.txt").expect("failed to load input file");

        let result = day3::part1::solution(input_string);
        assert_eq!(result, 161);
    }

    #[test]
    fn day3_part1() {
        let input_string =
            fs::read_to_string("inputs/day3.txt").expect("failed to load input file");

        let result = day3::part1::solution(input_string);
        assert_eq!(result, 161085926);
    }

    #[test]
    fn day3_part2_ex() {
        let input_string =
            fs::read_to_string("inputs/day3_example_2.txt").expect("failed to load input file");

        let result = day3::part2::solution(input_string);
        assert_eq!(result, 48);
    }

    #[test]
    fn day3_part2() {
        let input_string =
            fs::read_to_string("inputs/day3.txt").expect("failed to load input file");

        let result = day3::part2::solution(input_string);
        assert_eq!(result, 82045421);
    }

    #[test]
    fn day4_part1_ex() {
        let input_string =
            fs::read_to_string("inputs/day4_example.txt").expect("failed to load input file");

        let result = day4::part1::solution(input_string);
        assert_eq!(result, 18);
    }

    #[test]
    fn day4_part1() {
        let input_string =
            fs::read_to_string("inputs/day4.txt").expect("failed to load input file");

        let result = day4::part1::solution(input_string);
        assert_eq!(result, 2613);
    }

    #[test]
    fn day4_part2_ex() {
        let input_string =
            fs::read_to_string("inputs/day4_example.txt").expect("failed to load input file");

        let result = day4::part2::solution(input_string);
        assert_eq!(result, 9);
    }

    #[test]
    fn day4_part2() {
        let input_string =
            fs::read_to_string("inputs/day4.txt").expect("failed to load input file");

        let result = day4::part2::solution(input_string);
        assert_eq!(result, 1905);
    }

    #[test]
    fn day5_part1_ex() {
        let input_string =
            fs::read_to_string("inputs/day5_example.txt").expect("failed to load input file");

        let result = day5::part1::solution(input_string);
        assert_eq!(result, 143);
    }

    #[test]
    fn day5_part1() {
        let input_string =
            fs::read_to_string("inputs/day5.txt").expect("failed to load input file");

        let result = day5::part1::solution(input_string);
        assert_eq!(result, 4957);
    }

    #[test]
    fn day5_part2_ex() {
        let input_string =
            fs::read_to_string("inputs/day5_example.txt").expect("failed to load input file");

        let result = day5::part2::solution(input_string);
        assert_eq!(result, 123);
    }

    #[test]
    fn day5_part2() {
        let input_string =
            fs::read_to_string("inputs/day5.txt").expect("failed to load input file");

        let result = day5::part2::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 6938);
    }

    #[test]
    fn day6_part1_ex() {
        let input_string =
            fs::read_to_string("inputs/day6_example.txt").expect("failed to load input file");

        let result = day6::part1::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 41);
    }

    #[test]
    fn day6_part1() {
        let input_string =
            fs::read_to_string("inputs/day6.txt").expect("failed to load input file");

        let result = day6::part1::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 4819);
    }

    #[test]
    fn day6_part2_ex() {
        let input_string =
            fs::read_to_string("inputs/day6_example.txt").expect("failed to load input file");

        let result = day6::part2::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 6);
    }

    #[test]
    fn day6_part2() {
        let input_string =
            fs::read_to_string("inputs/day6.txt").expect("failed to load input file");

        let result = day6::part2::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 1796);
    }

    #[test]
    fn day6_part2_parallel() {
        let input_string =
            fs::read_to_string("inputs/day6.txt").expect("failed to load input file");

        let result = day6::part2::solution_parallel(input_string);
        println!("{}", result);
        assert_eq!(result, 1796);
    }
}
