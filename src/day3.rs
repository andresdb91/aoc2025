use std::str::FromStr;

pub fn day3_1(input: Vec<&str>) -> u64 {
    input
        .iter()
        .map(|x| {
            x.chars()
                .map(|c| u64::from_str(&c.to_string()).unwrap())
                .collect::<Vec<u64>>()
        })
        .map(|bank| {
            let mut first = 0;
            let mut second = None;
            for battery in &bank[0..bank.len() - 1] {
                if *battery > first {
                    first = *battery;
                    second = None;
                } else if let Some(s) = second
                    && s >= battery
                {
                    continue;
                } else {
                    second = Some(battery);
                }
            }
            let last_battery = bank[bank.len() - 1];
            if let Some(s) = second
                && s >= &last_battery
            {
            } else {
                second = Some(&last_battery);
            }
            first * 10 + second.unwrap()
        })
        .sum()
}

pub fn day3_2(input: Vec<&str>) -> u64 {
    input
        .iter()
        .map(|x| {
            x.chars()
                .map(|c| u64::from_str(&c.to_string()).unwrap())
                .collect::<Vec<u64>>()
        })
        .map(|bank| {
            let mut picked_batteries = vec![];
            let mut remaining_batteries = 12;
            let mut first_index = 0;
            while remaining_batteries > 0 {
                let mut next_index = 0;
                let mut max_battery = 0;
                for (index, battery) in bank[first_index..=bank.len()-remaining_batteries].iter().enumerate() {
                    if battery > &max_battery {
                        max_battery = *battery;
                        next_index = index + 1;
                    }
                }
                picked_batteries.push(max_battery);
                first_index += next_index;
                remaining_batteries -= 1;
            }
            // println!("{:?} -> {:?}", bank, picked_batteries);
            picked_batteries.into_iter().reduce(|acc, x| acc*10 + x).unwrap()
        })
        .sum()
}
