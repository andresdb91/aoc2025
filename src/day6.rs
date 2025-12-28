use std::{str::FromStr, vec};

pub fn day6_1(input: Vec<&str>) -> u64 {
    let rows: Vec<Vec<u64>> = input[..input.len() - 1]
        .iter()
        .map(|x| {
            x.split_whitespace()
                .map(|x| u64::from_str(x).unwrap())
                .collect()
        })
        .collect();
    let ops: Vec<&str> = input.last().unwrap().split_whitespace().collect();
    let mut grand_total = 0;
    for i in 0..rows[0].len() {
        let op = ops[i];
        let mut problem_total = { if op == "*" { 1 } else { 0 } };
        for j in 0..rows.len() {
            match op {
                "+" => {
                    problem_total += rows[j][i];
                }
                "*" => {
                    problem_total *= rows[j][i];
                }
                _ => {}
            }
            grand_total += problem_total;
        }
    }
    grand_total
}

pub fn day6_2(input: Vec<&str>) -> u64 {
    let rows: Vec<Vec<char>> = input[..input.len() - 1].iter().map(|x| x.chars().collect()).collect();
    let ops: Vec<&str> = input.last().unwrap().split_whitespace().collect();
    let mut grand_total = 0;
    for i in 0..rows[0].len() {
        let op = ops[i];
        let mut problem_total = { if op == "*" { 1 } else { 0 } };
        for j in 0..rows.len() {
            match op {
                "+" => {
                    problem_total += rows[j][i];
                }
                "*" => {
                    problem_total *= rows[j][i];
                }
                _ => {}
            }
            grand_total += problem_total;
        }
    }
    grand_total
}
