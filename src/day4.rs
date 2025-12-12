use std::cmp::min;

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
                for v in j.saturating_sub(1)..=min(y - 1, j + 1) {
                    for u in i.saturating_sub(1)..=min(x - 1, i + 1) {
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
    // println!("{:?}", rolls_table);
    rolls
}

#[derive(Clone, Copy, PartialEq)]
enum RollState {
    None,
    Available,
    Blocked,
    ToRemove,
    Removed,
}

pub fn day4_2(input: Vec<&str>) -> u64 {
    let y = input.len();
    let x = input[0].len();
    let mut rolls_table: Vec<Vec<(RollState, u64)>> = vec![vec![(RollState::None, 0); x]; y];
    let mut total_rolls = 0;
    let mut first_pass = true;
    loop {
        let mut rolls = 0;
        for (j, row) in input.iter().enumerate() {
            for (i, pos) in row.chars().enumerate() {
                if pos == '.' || rolls_table[j][i].0 == RollState::Removed {
                    continue;
                }
                if rolls_table[j][i].0 == RollState::None {
                    if rolls_table[j][i].1 < 4 {
                        rolls_table[j][i].0 = RollState::Available;
                        rolls += 1;
                    } else {
                        rolls_table[j][i].0 = RollState::Blocked;
                    }
                } else if rolls_table[j][i].0 == RollState::Available {
                    rolls_table[j][i].0 = RollState::ToRemove;
                } else if rolls_table[j][i].0 == RollState::ToRemove {
                    rolls_table[j][i].0 = RollState::Removed;
                }
                for v in j.saturating_sub(1)..=min(y - 1, j + 1) {
                    for u in i.saturating_sub(1)..=min(x - 1, i + 1) {
                        if u != i || v != j {
                            if first_pass {
                                rolls_table[v][u].1 += 1;
                                if rolls_table[v][u].0 == RollState::Available && rolls_table[v][u].1 >= 4 {
                                    rolls_table[v][u].0 = RollState::Blocked;
                                    rolls -= 1;
                                }
                            } else {
                                if rolls_table[j][i].0 == RollState::ToRemove {
                                    rolls_table[v][u].1 = rolls_table[v][u].1.saturating_sub(1);
                                }
                                if rolls_table[v][u].0 == RollState::Blocked && rolls_table[v][u].1 < 4 {
                                    rolls_table[v][u].0 = RollState::Available;
                                    rolls += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        first_pass = false;
        if rolls == 0 {
            break;
        } else {
            total_rolls += rolls;
        }
    }
    total_rolls
}
