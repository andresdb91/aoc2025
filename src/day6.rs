use std::str::FromStr;

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
        let mut problem_total;
        if op == "*" {
            problem_total = 1;
        } else {
            problem_total = 0
        }
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
        }
        grand_total += problem_total;
    }
    grand_total
}

pub fn day6_2(input: Vec<&str>) -> u64 {
    let rows: Vec<Vec<char>> = input[..input.len() - 1]
        .iter()
        .map(|x| x.chars().collect())
        .collect();
    let ops: Vec<&str> = input.last().unwrap().split_whitespace().collect();

    let mut problem_idx = 0;
    let mut values: Vec<Vec<String>> = vec![vec![]];
    for i in 0..rows[0].len() {
        let mut col = vec![];
        for j in 0..rows.len() {
            col.push(rows[j][i]);
        }
        let col_val: String = col.iter().collect();
        if col_val.trim() != "" {
            values[problem_idx].push(col_val);
        } else {
            problem_idx += 1;
            values.push(vec![]);
        }
    }

    let mut grand_total = 0;
    let mut op_iter = ops.iter();
    for problem in values {
        let op = op_iter.next().unwrap();
            // Solve operation
            let mut result: u64;
            if *op == "*" {
                result = 1;
            } else {
                result = 0;
            }
            for arg in problem.iter().map(|x| u64::from_str(x.trim()).unwrap()) {
                if *op == "*" {
                    result *= arg;
                } else {
                    result += arg;
                }
            }
            grand_total += result;
    }
    grand_total
}
