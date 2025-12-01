use aoc_2025::*;

pub fn day00(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 00");


    let data = read_input(00, input_type, manual_name)?;

    for row in &data {
        println!("{:?}", row);
    }


    Ok(())
}