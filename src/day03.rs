use std::cmp::max;
use combinatorial::Combinations;
use aoc_2025::*;

pub fn day03(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 03");


    let data = read_input(03, input_type, manual_name)?;

    // brute force will come and bite us in the arse, but hey, that's half the fun
    let mut batteries: Vec<Vec<u32>> = Vec::new();
    for row in &data {
        // println!("{:?}", row);
        let battery_char = str_to_chars(row[0].as_str());
        batteries.push(battery_char.iter().map(|x| x.to_digit(10).unwrap()).collect());
    }
    // println!("{:?}", batteries);

    let mut max_jolt = 0;
    for battery in &batteries {
        let mut local_max = 0;
        // println!("{:?}", battery);
        for i in 0..battery.len()-1 {
            for j in i+1..battery.len() {
                if i != j {
                    local_max = max(local_max, (battery[i]*10)+battery[j]);
                }
            }
        }
        // println!("local max: {}", local_max);
        max_jolt += local_max;
    }

    println!("The max joltage we can get is {}", max_jolt);
    // 17412

    // cool, cool, cool, 12...
    // let's bring in combinatorial help!
    //ombinations::of_size(vec!['a', 'b', 'c'], 2);
    let mut big_max_jolt = 0;
    for battery in batteries {
        let mut local_max=0;
        // println!("{:?}", battery);
        let choose_12 = Combinations::of_size(0..battery.len(),12);
        // println!("{:?}", battery);
        // println!("{:?}", choose_12);
        for combo_id in choose_12 {
            // println!("{:?}", combo_id);
            // smush the numbers together
            let combo_string = combo_id.iter().filter_map(|i| Some(battery[*i].to_string())).collect::<String>();
            // println!("{:?}", combo_string);
            local_max = max(local_max, str_to_u64(&*combo_string));
        }
        big_max_jolt += local_max;

    }
    println!("The max joltage we can get with 12 batteries is {}", big_max_jolt);

    Ok(())
}