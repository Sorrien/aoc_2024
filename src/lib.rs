pub mod common;
pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day1;
    use crate::day10;
    use crate::day11;
    use crate::day12;
    use crate::day13;
    use crate::day14;
    use crate::day15;
    use crate::day16;
    use crate::day2;
    use crate::day3;
    use crate::day4;
    use crate::day5;
    use crate::day6;
    use crate::day7;
    use crate::day8;
    use crate::day9;

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

    #[test]
    fn day7_part1_ex() {
        let input_string =
            fs::read_to_string("inputs/day7_example.txt").expect("failed to load input file");

        let result = day7::part1::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 3749);
    }

    #[test]
    fn day7_part1() {
        let input_string =
            fs::read_to_string("inputs/day7.txt").expect("failed to load input file");

        let result = day7::part1::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 3312271365652);
    }

    #[test]
    fn day7_part2_ex() {
        let input_string =
            fs::read_to_string("inputs/day7_example.txt").expect("failed to load input file");

        let result = day7::part2::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 11387);
    }

    #[test]
    fn day7_part2() {
        let input_string =
            fs::read_to_string("inputs/day7.txt").expect("failed to load input file");

        let result = day7::part2::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 509463489296712);
    }

    #[test]
    fn day8_part1_ex() {
        let input_string =
            fs::read_to_string("inputs/day8_example.txt").expect("failed to load input file");

        let result = day8::part1::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 14);
    }

    #[test]
    fn day8_part1() {
        let input_string =
            fs::read_to_string("inputs/day8.txt").expect("failed to load input file");

        let result = day8::part1::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 423);
    }

    #[test]
    fn day8_part2_ex() {
        let input_string =
            fs::read_to_string("inputs/day8_example.txt").expect("failed to load input file");

        let result = day8::part2::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 34);
    }

    /*     #[test]
    fn day8_part2_ex_2() {
        let input_string =
            fs::read_to_string("inputs/day8_example_2.txt").expect("failed to load input file");

        let result = day8::part2::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 9);
    } */

    #[test]
    fn day8_part2() {
        let input_string =
            fs::read_to_string("inputs/day8.txt").expect("failed to load input file");

        let result = day8::part2::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 1287);
    }

    #[test]
    fn day9_part1_ex() {
        let input_string =
            fs::read_to_string("inputs/day9_example.txt").expect("failed to load input file");

        let result = day9::part1::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 1928);
    }

    #[test]
    fn day9_part1() {
        let input_string =
            fs::read_to_string("inputs/day9.txt").expect("failed to load input file");

        let result = day9::part1::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 6519155389266);
    }

    #[test]
    fn day9_part2_ex() {
        let input_string =
            fs::read_to_string("inputs/day9_example.txt").expect("failed to load input file");

        let result = day9::part2::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 2858);
    }

    #[test]
    fn day9_part2() {
        let input_string =
            fs::read_to_string("inputs/day9.txt").expect("failed to load input file");

        let result = day9::part2::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 6547228115826);
    }

    #[test]
    fn day10_part1_ex() {
        let input_string =
            fs::read_to_string("inputs/day10_example.txt").expect("failed to load input file");

        let result = day10::part1::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 36);
    }

    #[test]
    fn day10_part1() {
        let input_string =
            fs::read_to_string("inputs/day10.txt").expect("failed to load input file");

        let result = day10::part1::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 694);
    }

    #[test]
    fn day10_part2_ex() {
        let input_string =
            fs::read_to_string("inputs/day10_example.txt").expect("failed to load input file");

        let result = day10::part2::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 81);
    }

    #[test]
    fn day10_part2() {
        let input_string =
            fs::read_to_string("inputs/day10.txt").expect("failed to load input file");

        let result = day10::part2::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 1497);
    }

    #[test]
    fn day11_part1_ex() {
        let input_string =
            fs::read_to_string("inputs/day11_example.txt").expect("failed to load input file");

        let result = day11::part1::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 55312);
    }

    #[test]
    fn day11_part1() {
        let input_string =
            fs::read_to_string("inputs/day11.txt").expect("failed to load input file");

        let result = day11::part1::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 197357);
    }

    #[test]
    fn day11_part2() {
        let input_string =
            fs::read_to_string("inputs/day11.txt").expect("failed to load input file");

        let result = day11::part2::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 234568186890978);
    }

    #[test]
    fn day12_part1_ex() {
        let input_string =
            fs::read_to_string("inputs/day12_example.txt").expect("failed to load input file");

        let result = day12::part1::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 1930);
    }

    #[test]
    fn day12_part2_ex() {
        let input_string =
            fs::read_to_string("inputs/day12_example.txt").expect("failed to load input file");

        let result = day12::part2::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 1206);
    }

    #[test]
    fn day12_part2_ex_2() {
        let input_string =
            fs::read_to_string("inputs/day12_example2.txt").expect("failed to load input file");

        let result = day12::part2::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 80);
    }

    #[test]
    fn day12_part2() {
        let input_string =
            fs::read_to_string("inputs/day12.txt").expect("failed to load input file");

        let result = day12::part2::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 834828);
    }

    #[test]
    fn day13_part1_ex() {
        let input_string =
            fs::read_to_string("inputs/day13_example.txt").expect("failed to load input file");

        let result = day13::part1::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 480);
    }

    #[test]
    fn day13_part1() {
        let input_string =
            fs::read_to_string("inputs/day13.txt").expect("failed to load input file");

        let result = day13::part1::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 35729);
    }

    #[test]
    fn day13_part2() {
        let input_string =
            fs::read_to_string("inputs/day13.txt").expect("failed to load input file");

        let result = day13::part2::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 88584689879723);
    }

    #[test]
    fn day14_part1_ex() {
        let input_string =
            fs::read_to_string("inputs/day14_example.txt").expect("failed to load input file");

        let result = day14::part1::solution(input_string, 11, 7);
        println!("{}", result);
        assert_eq!(result, 12);
    }

    #[test]
    fn day14_part1() {
        let input_string =
            fs::read_to_string("inputs/day14.txt").expect("failed to load input file");

        let result = day14::part1::solution(input_string, 101, 103);
        println!("{}", result);
        assert_eq!(result, 225648864);
    }

    #[test]
    fn day14_part2() {
        let input_string =
            fs::read_to_string("inputs/day14.txt").expect("failed to load input file");

        let result = day14::part2::solution(input_string, 101, 103);
        println!("{}", result);
        assert_eq!(result, 7847);
    }

    #[test]
    fn day15_part1_ex() {
        let input_string =
            fs::read_to_string("inputs/day15_example.txt").expect("failed to load input file");

        let result = day15::part1::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 10092);
    }

    #[test]
    fn day15_part1() {
        let input_string =
            fs::read_to_string("inputs/day15.txt").expect("failed to load input file");

        let result = day15::part1::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 1527563);
    }

    #[test]
    fn day15_part2_ex() {
        let input_string =
            fs::read_to_string("inputs/day15_example.txt").expect("failed to load input file");

        let result = day15::part2::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 9021);
    }

    #[test]
    fn day15_part2() {
        let input_string =
            fs::read_to_string("inputs/day15.txt").expect("failed to load input file");

        let result = day15::part2::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 1521635);
    }

    #[test]
    fn day16_part1_ex() {
        let input_string =
            fs::read_to_string("inputs/day16_example.txt").expect("failed to load input file");

        let result = day16::part1::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 7036);
    }

    #[test]
    fn day16_part1_ex_2() {
        let input_string =
            fs::read_to_string("inputs/day16_example_2.txt").expect("failed to load input file");

        let result = day16::part1::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 11048);
    }

    #[test]
    fn day16_part1() {
        let input_string =
            fs::read_to_string("inputs/day16.txt").expect("failed to load input file");

        let result = day16::part1::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 72428);
    }

    #[test]
    fn day16_part2_ex() {
        let input_string =
            fs::read_to_string("inputs/day16_example.txt").expect("failed to load input file");

        let result = day16::part2::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 45);
    }

    #[test]
    fn day16_part2_ex_2() {
        let input_string =
            fs::read_to_string("inputs/day16_example_2.txt").expect("failed to load input file");

        let result = day16::part2::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 64);
    }

    #[test]
    fn day16_part2() {
        let input_string =
            fs::read_to_string("inputs/day16.txt").expect("failed to load input file");

        let result = day16::part2::solution(input_string);
        println!("{}", result);
        assert_eq!(result, 456);
    }
}
