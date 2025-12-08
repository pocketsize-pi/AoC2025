use aoc_2025::*;

pub fn day05(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 05");


    let data = read_input(05, input_type, manual_name)?;
    let mut ingredient_list = Vec::new();
    let mut fresh_ranges = Vec::new();
    let mut process_ingredients = false;

    for row in &data {
        // println!("{:?}", row);
        if row.is_empty() {
            process_ingredients = true; // ranges first
        }
        else {
            if process_ingredients {
                let ingr = str_to_u64(&row[0]);
                ingredient_list.push(ingr);
            }
            else {
                let range_str : Vec<&str> = row[0].split("-").collect();
                let range = (str_to_u64(range_str[0]), str_to_u64(range_str[1]));
                fresh_ranges.push(range);
            }
        }

    }
    ingredient_list.sort();
    fresh_ranges.sort();
    // println!("{:?}", ingredient_list);
    // println!("{:?}", fresh_ranges);

    let mut total_fresh = 0;

    // this is probably not going to work with the full input *shrug*
    for ingredient in ingredient_list {
        for range in &fresh_ranges {
            if (ingredient >= range.0) & (ingredient <= range.1) {
                total_fresh += 1;
                break;
            }
        }
    }

    println!("The total fresh ingredients is {}", total_fresh);


    Ok(())
}