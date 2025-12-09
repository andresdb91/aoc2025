use std::fs;

use aoc2025;

#[test]
fn day4_part1_test() {
    let file = fs::read_to_string("input_test/day4.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result: u64 = 13;
    assert_eq!(aoc2025::day4::day4_1(data), result)
}

#[test]
#[ignore]
fn day4_part1() {
    let file = fs::read_to_string("input/day4.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result = aoc2025::day4::day4_1(data);
    println!("{}", result);
}

#[test]
#[ignore]
fn day4_part2_test() {
    let file = fs::read_to_string("input_test/day4.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result: u64 = 3121910778619;
    assert_eq!(aoc2025::day4::day4_2(data), result)
}

#[test]
#[ignore]
fn day4_part2() {
    let file = fs::read_to_string("input/day4.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result = aoc2025::day4::day4_2(data);
    println!("{}", result);
}
