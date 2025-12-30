use std::fs;

use aoc2025;

const PART1_TEST_RESULT: u64 = 21;
const PART2_TEST_RESULT: u64 = 40;

#[test]
fn day7_part1_test() {
    let file = fs::read_to_string("input_test/day7.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result: u64 = PART1_TEST_RESULT;
    assert_eq!(aoc2025::day7::day7_1(data), result)
}

#[test]
// #[ignore]
fn day7_part1() {
    let file = fs::read_to_string("input/day7.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result = aoc2025::day7::day7_1(data);
    println!("{}", result);
}

#[test]
// #[ignore]
fn day7_part2_test() {
    let file = fs::read_to_string("input_test/day7.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result: u64 = PART2_TEST_RESULT;
    assert_eq!(aoc2025::day7::day7_2(data), result)
}

#[test]
#[ignore]
fn day7_part2() {
    let file = fs::read_to_string("input/day7.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result = aoc2025::day7::day7_2(data);
    println!("{}", result);
}
