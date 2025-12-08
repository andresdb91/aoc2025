use std::str::FromStr;

pub fn day1_1(input: Vec<&str>) -> u32 {
    let mut result = 0;
    if input.iter().map(|x| {
        let (op, val) = x.split_at(1);
        if op == "L" {
            100 - (i32::from_str(val).unwrap() % 100)
        } else {
            i32::from_str(val).unwrap()
        }
    }).reduce(|acc, x| {
        if acc == 50 {
            result += 1;
        }
        (acc + x) % 100
    }).unwrap() == 50 {
        result += 1;
    }
    result
}

pub fn day1_2(input: Vec<&str>) -> u32 {
    let mut pos = 50;
    let mut result: u32 = 0;
    for x in input {
        let (op, val) = x.split_at(1);
        let rot = i32::from_str(val).unwrap();
        if op == "R" {
            let new_pos = pos + rot;
            result += ((new_pos)/100).unsigned_abs();
            pos = (new_pos) % 100;
            // println!("[{}] {} -> {}", result, rot, pos);
        } else {
            let new_pos = pos - rot;
            if pos > 0 && new_pos <= 0 {
                result += 1;
            }
            pos = ((new_pos % 100) + 100) % 100;
            result += (new_pos / 100).unsigned_abs();
            // println!("[{}] {} -> {}", result, -rot, pos);
        }
    };
    result
}
