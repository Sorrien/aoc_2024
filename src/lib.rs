pub mod day1;
pub mod day2;

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day1;
    use crate::day2;

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
}
