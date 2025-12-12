use std::str::FromStr;

pub fn day5_1(input: Vec<&str>) -> u64 {
    let (fresh_ranges_raw, ingredients_raw) = input.split_at(input.iter().position(|x| *x == "").unwrap());
    let ingredients = &ingredients_raw[1..];
    let mut fresh_ranges: Vec<(u64, u64)> = vec![];
    for ranges_raw in fresh_ranges_raw {
        let (raw_start, raw_end) = (u64::from_str(ranges_raw.split("-").collect::<Vec<&str>>()[0]).unwrap(), u64::from_str(ranges_raw.split("-").collect::<Vec<&str>>()[1]).unwrap());
        for (range_start, range_end) in fresh_ranges.iter_mut() {
            if raw_start < *range_start && raw_end >= *range_start {
                *range_start = raw_start;
            }
        }
    }
    3
}

pub fn day5_2(input: Vec<&str>) -> u64 {
    0
}
