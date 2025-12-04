use std::cmp::max;
// use combinatorial::Combinations;
// use itertools::Itertools;
use combination::*;
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
    // let's bring in combinatorial help
    let mut tried = Vec::new();

    let mut big_max_jolt = 0;
    let mut n=0;
    for battery in batteries {
        let mut local_max=0;
        n+=1;
        println!("Battery {}",n);
        // println!("{:?}", battery);
        // let choose_12 = Combinations::of_size(0..battery.len(),12);
        // // println!("{:?}", battery);
        // // println!("{:?}", choose_12);
        // for combo_id in choose_12 {
        //     // println!("{:?}", combo_id);
        //     // smush the numbers together
        //     let combo_string = combo_id.iter().filter_map(|i| Some(battery[*i].to_string())).collect::<String>();
        //     // println!("{:?}", combo_string);
        //     local_max = max(local_max, str_to_u64(&*combo_string));
        // }

        // println!("{:?}", battery);
        // I couldn't get combinatorial to work
        // oh! generate combination is what is keeping our program stuck!
        let mut choose_12: Vec<Vec<u32>> = combine::from_vec_at(&battery, 12);
        // let choose_12: Vec<Vec<u32>> = battery.into_iter().combinations(12).collect();
        // remove duplicates, there must be many
        // let small_choose_12 = choose_12.into_iter().unique();
        // println!("orig {:?}", choose_12);
        choose_12.sort();
        // println!("sort {:?}", choose_12);
        choose_12.dedup();
        // println!("dedup {:?}", choose_12);
        println!(" ({} combinations)", choose_12.len());


        // println!("12 {:?}", choose_12);
        for combo in choose_12 {
            if !tried.contains(&combo) {
                // println!("{:?}", combo_id);
                // smush the numbers together
                let combo_string = combo.iter().map(|n| n.to_string()).collect::<String>();
                //.iter().filter_map(|i| Some(battery[*i].to_string())).collect::<String>();
                // println!("{:?}", combo_string);
                local_max = max(local_max, str_to_u64(&*combo_string));
                tried.push(combo);
            }

        }


        big_max_jolt += local_max;

    }
    println!("The max joltage we can get with 12 batteries is {}", big_max_jolt);

    Ok(())
}