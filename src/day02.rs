use divisors::get_divisors;
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

    for pair in &ranges_list {
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

    // ok, let's still brute force part 2
    num_invalid = 0;
    invalid_sum = 0;

    for pair in &ranges_list {
        for id   in pair.0..pair.1+1 {
            // convert number to string
            let id_str = id.to_string();
            // get divisors - I could do this manually, but there is a crate for it
            let mut divs = get_divisors(id_str.len());
            // note that this does not give us one! so we need to make that ourselves
            divs.insert(0,1);
            // println!("{id} -----");

            for div in divs {
                // println!(" div {div}");
                let min_sxn = id_str[..div].to_string();
                let num_reps = id_str.len()/div;
                let actual_reps = id_str.matches(&min_sxn).count();
                // println!(" Min sxn: {:?}, num_reps: {}, actual_reps: {}", min_sxn, num_reps, actual_reps);
                if (num_reps > 1) & (num_reps == actual_reps) {
                    num_invalid += 1;
                    invalid_sum += id;
                    // debug
                    // println!("invalid {id}");
                    // don't count the same one multiple times!
                    break;
                }
            }
        }
    }

    println!("There are now {} invalid ids, for a sum of {}", num_invalid, invalid_sum);
    // 43872163557


    Ok(())
}