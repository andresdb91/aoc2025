use std::fs;

use aoc2025;

#[test]
fn aoc2025_part1_test() {
    let file = fs::read_to_string("input_test/day1.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result: u32 = 3;
    assert_eq!(aoc2025::day1::day1_1(data), result)
}

#[test]
// #[ignore]
fn aoc2025_part1() {
    let file = fs::read_to_string("input/day1.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result = aoc2025::day1::day1_1(data);
    println!("{}", result);
}

#[test]
// #[ignore]
fn aoc2025_part2_test() {
    let file = fs::read_to_string("input_test/day1.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result: u32 = 6;
    assert_eq!(aoc2025::day1::day1_2(data), result)
}

#[test]
// #[ignore]
fn aoc2025_part2() {
    let file = fs::read_to_string("input/day1.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result = aoc2025::day1::day1_2(data);
    println!("{}", result);
}
