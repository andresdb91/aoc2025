use std::fs;

use aoc2025;

const PART1_TEST_RESULT: u64 = 4277556;
const PART2_TEST_RESULT: u64 = 3263827;

#[test]
fn day6_part1_test() {
    let file = fs::read_to_string("input_test/day6.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result: u64 = PART1_TEST_RESULT;
    assert_eq!(aoc2025::day6::day6_1(data), result)
}

#[test]
// #[ignore]
fn day6_part1() {
    let file = fs::read_to_string("input/day6.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result = aoc2025::day6::day6_1(data);
    println!("{}", result);
}

#[test]
// #[ignore]
fn day6_part2_test() {
    let file = fs::read_to_string("input_test/day6.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result: u64 = PART2_TEST_RESULT;
    assert_eq!(aoc2025::day6::day6_2(data), result)
}

#[test]
// #[ignore]
fn day6_part2() {
    let file = fs::read_to_string("input/day6.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result = aoc2025::day6::day6_2(data);
    println!("{}", result);
}
