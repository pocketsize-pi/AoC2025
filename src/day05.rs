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
    // 640

    // -------------------------------------------------------------

    fn compare_two_pairs (this_range: (u64, u64), next_range: (u64, u64)) -> Option<((u64,u64),bool)> {
        // let output : Option<(u64,u64)>;
        if (next_range.0..next_range.1+1).contains(&this_range.0) &
            (next_range.0..next_range.1+1).contains(&this_range.1) {
            // this fully contained by next
            None
        }
        else if (this_range.0..this_range.1+1).contains(&next_range.0) &
            (this_range.0..this_range.1+1).contains(&next_range.1) {
            // next fully contained by this (start with same number!)
            Some((this_range, true))
        }
        else if (this_range.0..this_range.1+1).contains(&next_range.0) &
            (next_range.0..next_range.1+1).contains(&this_range.1) {
            // starts in this and ends in next
            // print!("starts in this and ends in next:");
            Some(((this_range.0,next_range.1),true))
        }
        else if (next_range.0..next_range.1+1).contains(&this_range.0) &
            (this_range.0..this_range.1+1).contains(&next_range.1) {
            // starts in next and ends in this
            // print!("xxxxxxstarts in this and ends in next:");
            Some(((this_range.0, next_range.1), true))
        }
            //MISSING IF THIS CONTAINS NEXT!
        else {
            // nothing to merge, push;
            Some((this_range, false))
        }
    }


    let mut possible_fresh = fresh_ranges.clone();
    // let mut i = 0;
    loop {

        let mut new_fresh = Vec::new();
        let mut skip_next = false;
        // println!("---------{:?}", possible_fresh);
        for r in 0..possible_fresh.len()-1 {

            if skip_next {
                skip_next = false;
                continue;
            }
            // ranges is sorted, so we just need to build a list that is good and has no overlap
            let this_range = possible_fresh[r];
            // process last point
            let next_range = possible_fresh[r+1];

            // println!("{:?}, {:?}", this_range, next_range);
            let reduction = compare_two_pairs(this_range, next_range);

            match reduction {
                None => {}
                Some((new_range,skip)) => {
                    new_fresh.push(new_range);
                    // println!("{:?}, skip: {skip}", new_range);
                    if skip {
                        skip_next = true;
                    }
                }
            }
        }
        // println!("  {:?}", new_fresh);

        // sort out last pair
        let reduction = compare_two_pairs(*new_fresh.last().unwrap(), *possible_fresh.last().unwrap());

        match reduction {
            None => {
                // println!("replacing last fresh!");
                *new_fresh.last_mut().unwrap() = *possible_fresh.last().unwrap() }
            Some((new_range, skip)) => {
                if skip {
                    new_fresh.push(new_range);
                }
                else {
                    new_fresh.push(*possible_fresh.last().unwrap());
                }
            }
        }

        // println!("  {:?}", new_fresh);

        if new_fresh == possible_fresh {
            // println!("break?");
            break;
        }
        possible_fresh = new_fresh.clone();
        possible_fresh.sort(); // just in case

        // i += 1;
        // if i >= 10 {
        //     break;
        // }

    }

    // println!("{:?}", possible_fresh);

    total_fresh = 0;
    for range in possible_fresh {
        total_fresh += (range.0..range.1+1).count();
    }

    println!("The total possible fresh ingredients is {}", total_fresh);
    // 11753977975152 too low
    // 375947484460456 too high
    // 365804144481581 hurrah!



    Ok(())
}