use std::str::FromStr;

pub fn day2_1(input: Vec<&str>) -> u64 {
    let ranges: Vec<(u64, u64)> = input[0].split(',').map(|x| {
        let _range = x.split('-').map(|x| u64::from_str(x).unwrap()).collect::<Vec<u64>>();
        // println!("[{} - {}]", _range[0], _range[1]);
        (_range[0], _range[1])
    }).collect();
    let mut result = 0;
    for (rstart, rend) in ranges {
        // println!("[{} - {}]", rstart, rend);
        for i in rstart..=rend {
            let s = i.to_string();
            if s.len() % 2 == 0 {
                let (sa, sb) = s.split_at(s.len()/2);
                if sa == sb {
                    // println!("{} | {}", sa, sb);
                    result += i;
                }
            }
        }
    }
    result
}

pub fn day2_2(input: Vec<&str>) -> u64 {
    let ranges: Vec<(u64, u64)> = input[0].split(',').map(|x| {
        let _range = x.split('-').map(|x| u64::from_str(x).unwrap()).collect::<Vec<u64>>();
        // println!("[{} - {}]", _range[0], _range[1]);
        (_range[0], _range[1])
    }).collect();
    let mut result = 0;
    for (rstart, rend) in ranges {
        // println!("[{} - {}]", rstart, rend);
        for i in rstart..=rend {
            let s = i.to_string();
            for pattern_length in 1..=s.len()/2 {
                if s.len() % pattern_length == 0 {
                    let sa = &s[0..pattern_length];
                    let mut ss_match = true;
                    for si in 1..(s.len()/pattern_length) {
                        let sb = &s[si*pattern_length..(si+1)*pattern_length];
                        if sa != sb {
                            ss_match = false;
                            break;
                        }
                    }
                    if ss_match {
                        // println!("{}: {}", i, sa);
                        result += i;
                        break;
                    }
                }
            }
        }
    }
    result
}
