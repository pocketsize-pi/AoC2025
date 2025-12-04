use std::char::from_digit;
use std::cmp::max;
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

    fn building_one_by_one (battery: Vec<u32>,
                            start_id: usize,
                            searching_for: usize,
                            mut partial_list: String) -> String {
        //debug

        let end_search = battery.len() - (12 - searching_for) + 1;

        // println!("Looking for {searching_for}, current word {:?}", partial_list);
        // println!("Search space: {:?}", battery[start_id..end_search].to_vec());

        let this_max = battery[start_id..end_search].iter().max().unwrap();
        let this_max_id = battery[start_id..end_search].into_iter().position(|x| x == this_max).unwrap();

        partial_list.push(from_digit(*this_max, 10).unwrap());
        if partial_list.len() == 12 {
            partial_list
        }
        else {
            building_one_by_one(battery, this_max_id+start_id+1,searching_for+1, partial_list)
        }

    }


    // cool, cool, cool, 12...
    let mut big_max_jolt = 0;
    // let mut n=0;
    for battery in batteries {
        // let mut local_max=0;
        // n+=1;
        // println!("Battery {}",n);
        // println!("{:?}", battery);
        // no, we build the number one number at a time, finding the max max number
        let smart_build = building_one_by_one(battery, 0, 0, "".parse().unwrap());
        // println!("Smart build: {:?}", smart_build);

        big_max_jolt += str_to_u64(&*smart_build);

    }
    println!("The max joltage we can get with 12 batteries is {}", big_max_jolt);
    // 172681562473501 hurrah!

    Ok(())
}