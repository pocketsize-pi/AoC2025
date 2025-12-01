use aoc_2025::*;
use modular::*;

pub fn day01(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 01");

    const CIRCLE_LIMIT : u32 = 100;

    let data = read_input(01, input_type, manual_name)?;
    let mut list_movements = Vec::new();

    for row in &data {
        // println!("{:?}", row);
        let dir = row[0].chars().nth(0).unwrap();
        // println!("{:?}", dir);
        let number : Modulo = (str_to_i32(&row[0][1..])).to_modulo(CIRCLE_LIMIT);
        // println!("{number}");
        list_movements.push((dir, number));
    }

    // This is a task for modular numbers if I ever saw one
    // Ada has modular numbers (love you), but Rust doesn't have them built in,
    // so we are bringing in a crate. I know the maths.

    let mut dial = modulo!(50,CIRCLE_LIMIT); // it helps so much if we get the right starting position
    let mut num_zero = 0;

    for (dir, number) in list_movements {

        if dir == 'R' {
            dial = dial + number;
        }
        else {
            dial = dial - number;
        }
        // debug print
        // println!("dir: {}, offset: {}, final: {}", dir, number, dial);

        // check if we are at 0
        if dial == modulo!(0,CIRCLE_LIMIT) {
            num_zero += 1;
        }
    }

    println!("We go through zero {} times", num_zero);
    // 1172

    Ok(())
}