use std::cmp::{min, max};

pub fn day7_1(input: Vec<&str>) -> u64 {
    let mut rows: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect()).collect();
    let row_len = rows[0].len();
    let mut splits = 0;
    for j in 1..rows.len() {
        for i in 0..row_len {
            match rows[j][i] {
                '^' => {
                    // Split
                    if rows[j-1][i] == '|' || rows[j-1][i] == 'S' {
                        rows[j][max(0, i-1)] = '|';
                        rows[j][min(row_len, i+1)] = '|';
                        splits += 1;
                    }
                }
                '.' => {
                    // Copy
                    rows[j][i] = rows[j-1][i]
                }
                _ => {}
            }
        }
    }
    splits
}

pub fn day7_2(input: Vec<&str>) -> u64 {
    let rows: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect()).collect();
    let row_len = rows[0].len();
    let row_num = rows.len();
    let mut multiverse = vec![(1, rows.clone())];
    let mut timelines = 0;
    let mut current = 0;
    loop {
        for j in multiverse[current].0..row_num {
            for i in 0..row_len {
                match multiverse[current].1[j][i] {
                    '^' => {
                        // Split
                        if multiverse[current].1[j-1][i] == '|' || multiverse[current].1[j-1][i] == 'S' {
                            timelines += 1;
                            multiverse.push((j+1, multiverse[current].1.clone()));
                            multiverse[current].1[j][max(0, i-1)] = '|';
                            multiverse[timelines].1[j][min(row_len, i+1)] = '|';
                        }
                    }
                    '.' => {
                        // Copy
                        multiverse[current].1[j][i] = multiverse[current].1[j-1][i]
                    }
                    _ => {}
                }
            }
        }
        if current < timelines {
            current += 1
        } else {
            break;
        }
    }
    (timelines + 1).try_into().unwrap()
}
