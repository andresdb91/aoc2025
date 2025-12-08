use std::fs;

use aoc2025;

#[test]
fn day2_part1_test() {
    let file = fs::read_to_string("input_test/day2.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result: u64 = 1227775554;
    assert_eq!(aoc2025::day2::day2_1(data), result)
}

#[test]
// #[ignore]
fn day2_part1() {
    let file = fs::read_to_string("input/day2.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result = aoc2025::day2::day2_1(data);
    println!("{}", result);
}

#[test]
// #[ignore]
fn day2_part2_test() {
    let file = fs::read_to_string("input_test/day2.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result: u64 = 4174379265;
    assert_eq!(aoc2025::day2::day2_2(data), result)
}

#[test]
// #[ignore]
fn day2_part2() {
    let file = fs::read_to_string("input/day2.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result = aoc2025::day2::day2_2(data);
    println!("{}", result);
}
