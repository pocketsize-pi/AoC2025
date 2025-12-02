use aoc_2025::*;

pub fn day02(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 02");


    let data = read_input(02, input_type, manual_name)?;
    let mut ranges_list = Vec::new();

    for row in &data {
        // println!("{:?}", row);
        // split by comma, then by dash
        let ranges : Vec<&str> = row[0].split(",").collect();
        for range in ranges {
            let pair_str: Vec<&str> = range.split("-").collect();
            let pair_int = (str_to_i64(pair_str[0]), str_to_i64(pair_str[1]));
            ranges_list.push(pair_int);        }
    }
    // println!("{:?}", ranges_list);

    let mut num_invalid = 0;
    let mut invalid_sum = 0;

    for pair in ranges_list {
        for id in pair.0..pair.1+1 {
            // convert number to string
            let id_str = id.to_string();
            // valid if odd number of chars
            if id_str.len() % 2 == 0 { // even
                // split into two, and compare both halves
                if id_str[..id_str.len()/2] == id_str[id_str.len()/2..] {
                    num_invalid += 1;
                    invalid_sum += id;
                    // debug
                    // println!("{id}");
                }
            }
        }

    }

    println!("There are {} invalid ids, for a sum of {}", num_invalid, invalid_sum);
    // 30323879646 yay


    Ok(())
}