use std::{str::FromStr, usize};

pub fn day5_1(input: Vec<&str>) -> u64 {
    let (fresh_ranges_raw, ingredients_raw) =
        input.split_at(input.iter().position(|x| *x == "").unwrap());
    let ingredients = &ingredients_raw[1..];
    let mut fresh_ingredients = 0;
    for i in ingredients.iter().map(|x| u64::from_str(x).unwrap()) {
        for fresh_range in fresh_ranges_raw {
            let (range_start, range_end) = (
                u64::from_str(fresh_range.split("-").collect::<Vec<&str>>()[0]).unwrap(),
                u64::from_str(fresh_range.split("-").collect::<Vec<&str>>()[1]).unwrap(),
            );
            if range_start <= i && i <= range_end {
                fresh_ingredients += 1;
                break;
            }
        }
    }
    fresh_ingredients
}

pub fn day5_2(input: Vec<&str>) -> u64 {
    let mut fresh_ranges: Vec<(u64, u64)> = input.split_at(input.iter().position(|x| *x == "").unwrap()).0.iter().map(|x| (
                u64::from_str(x.split("-").collect::<Vec<&str>>()[0]).unwrap(),
                u64::from_str(x.split("-").collect::<Vec<&str>>()[1]).unwrap(),
            )).collect();
    let mut fresh_ingredients = 0;
    let mut rounds = fresh_ranges.len();
    let mut prev_size = usize::MAX;
    loop {
        let mut new_range_start = 0;
        let mut new_range_end = 0;
        let mut remove_range = vec![];
        for (i, &(range_start, range_end)) in fresh_ranges.iter().enumerate() {
            if new_range_start == 0 && new_range_end == 0 {
                new_range_start = range_start;
                new_range_end = range_end;
                remove_range.push(i);
                continue;
            }
            if range_start <= new_range_start && range_end >= new_range_start - 1 {
                new_range_start = range_start;
                remove_range.push(i);
            }
            if range_start <= new_range_end + 1 && range_end >= new_range_end {
                new_range_end = range_end;
                if !remove_range.contains(&i) {
                    remove_range.push(i);
                }
            }
        }
        for rr in remove_range {
            fresh_ranges[rr] = (0, 0);
        }
        fresh_ranges.retain(|x| *x != (0, 0));
        fresh_ranges.push((new_range_start, new_range_end));
        if fresh_ranges.len() == prev_size {
            rounds -= 1;
        } else {
            rounds = fresh_ranges.len();
            prev_size = fresh_ranges.len();
        }
        if rounds == 0 {
            break;
        }
    }
    // println!("{:?}", fresh_ranges);
    for (fr_start, fr_end) in fresh_ranges {
        fresh_ingredients += fr_end - fr_start + 1;
    }
    // println!("{:?}", fresh_ingredients);
    fresh_ingredients
}
