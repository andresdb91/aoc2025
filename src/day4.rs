use std::cmp::{max, min};

pub fn day4_1(input: Vec<&str>) -> u64 {
    let y = input.len();
    let x = input[0].len();
    let mut rolls_table: Vec<Vec<(bool, u64)>> = vec![vec![(false, 0); x]; y];
    let mut rolls = 0;
    for (j, row) in input.iter().enumerate() {
        for (i, pos) in row.chars().enumerate() {
            if pos == '@' {
                if rolls_table[j][i].1 < 4 {
                    rolls_table[j][i].0 = true;
                    rolls += 1;
                }
                for v in j.wrapping_sub(1)..=min(y-1, j+1) {
                    for u in i.wrapping_sub(1)..=min(x-1, i+1) {
                        if u != i || v != j {
                            rolls_table[v][u].1 += 1;
                            if rolls_table[v][u].0 && rolls_table[v][u].1 >= 4 {
                                rolls_table[v][u].0 = false;
                                rolls -= 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", rolls_table);
    rolls
}

pub fn day4_2(input: Vec<&str>) -> u64 {
    0
}
