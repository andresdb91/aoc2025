use std::fs;

use aoc2025;

#[test]
fn day3_part1_test() {
    let file = fs::read_to_string("input_test/day3.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result: u64 = 357;
    assert_eq!(aoc2025::day3::day3_1(data), result)
}

#[test]
// #[ignore]
fn day3_part1() {
    let file = fs::read_to_string("input/day3.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result = aoc2025::day3::day3_1(data);
    println!("{}", result);
}

#[test]
// #[ignore]
fn day3_part2_test() {
    let file = fs::read_to_string("input_test/day3.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result: u64 = 3121910778619;
    assert_eq!(aoc2025::day3::day3_2(data), result)
}

#[test]
// #[ignore]
fn day3_part2() {
    let file = fs::read_to_string("input/day3.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result = aoc2025::day3::day3_2(data);
    println!("{}", result);
}
