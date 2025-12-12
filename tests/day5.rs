use std::fs;

use aoc2025;

#[test]
fn day5_part1_test() {
    let file = fs::read_to_string("input_test/day5.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result: u64 = 3;
    assert_eq!(aoc2025::day5::day5_1(data), result)
}

#[test]
#[ignore]
fn day5_part1() {
    let file = fs::read_to_string("input/day5.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result = aoc2025::day5::day5_1(data);
    println!("{}", result);
}

#[test]
#[ignore]
fn day5_part2_test() {
    let file = fs::read_to_string("input_test/day5.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result: u64 = 43;
    assert_eq!(aoc2025::day5::day5_2(data), result)
}

#[test]
#[ignore]
fn day5_part2() {
    let file = fs::read_to_string("input/day5.txt").unwrap();
    let data: Vec<&str> = file.split_terminator("\n").collect();
    let result = aoc2025::day5::day5_2(data);
    println!("{}", result);
}
