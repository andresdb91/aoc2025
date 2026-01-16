use std::cmp::{max, min};

pub fn day7_1(input: Vec<&str>) -> u64 {
    let mut rows: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect()).collect();
    let row_len = rows[0].len();
    let mut splits = 0;
    for j in 1..rows.len() {
        for i in 0..row_len {
            match rows[j][i] {
                '^' => {
                    // Split
                    if rows[j - 1][i] == '|' || rows[j - 1][i] == 'S' {
                        rows[j][max(0, i - 1)] = '|';
                        rows[j][min(row_len, i + 1)] = '|';
                        splits += 1;
                    }
                }
                '.' => {
                    // Copy
                    rows[j][i] = rows[j - 1][i]
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
    // let row_num = rows.len();
    let mut multiverse = vec![rows.clone()];
    let mut timelines = 0;
    let mut current = 0;
    let start = std::time::Instant::now();
    loop {
        for j in 1..multiverse[current].len() {
            for i in 0..row_len {
                match multiverse[current][j][i] {
                    '^' => {
                        // Split
                        if multiverse[current][j - 1][i] == '|'
                            || multiverse[current][j - 1][i] == 'S'
                        {
                            timelines += 1;
                            multiverse.insert(current + 1, multiverse[current][j..].to_vec());
                            multiverse[current][j][max(0, i - 1)] = '|';
                            multiverse[timelines][0][min(row_len, i + 1)] = '|';
                        }
                    }
                    '.' => {
                        // Copy
                        multiverse[current][j][i] = multiverse[current][j - 1][i]
                    }
                    _ => {}
                }
            }
        }
        if current < timelines {
            current += 1;
            if current % 200000 == 0 {
                let elapsed = start.elapsed();
                println!("Scanning timeline {} - Elapsed: {:?}", current, elapsed)
            }
        } else {
            let elapsed = start.elapsed();
            println!("Scanning finished - Elapsed: {:?}", elapsed);
            break;
        }
    }
    (timelines + 1).try_into().unwrap()
}
