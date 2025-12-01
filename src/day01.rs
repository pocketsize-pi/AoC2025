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
        // let number : Modulo = (str_to_i32(&row[0][1..])).to_modulo(CIRCLE_LIMIT);
        let number : i32 = str_to_i32(&row[0][1..]);
        // println!("{number}");
        list_movements.push((dir, number));
    }

    // This is a task for modular numbers if I ever saw one
    // Ada has modular numbers (love you), but Rust doesn't have them built in,
    // so we are bringing in a crate. I know the maths.

    let mut dial = modulo!(50,CIRCLE_LIMIT); // it helps so much if we get the right starting position
    let mut num_zero = 0;

    for (dir, number) in &list_movements {


        if *dir == 'R' {
            dial = dial + modulo!(number, CIRCLE_LIMIT);
        }
        else {
            dial = dial - modulo!(number, CIRCLE_LIMIT);
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

    // for part 2, we have removed the built-in modulo so that we have access to the "raw" number
    // we build the number non-modular
    // then we brute force, 'cause nice didn't work!

    dial = modulo!(50,CIRCLE_LIMIT);
    num_zero = 0;
    for (dir, number) in &list_movements {
        // move full blast
        if *dir == 'R' {
            for _i in 0..*number {
                dial = dial + modulo!(1,CIRCLE_LIMIT);
                if dial == modulo!(0,CIRCLE_LIMIT) {
                    num_zero += 1;
                }
            }
        }
        else {
            for _i in 0..*number {
                dial = dial - modulo!(1,CIRCLE_LIMIT);
                if dial == modulo!(0,CIRCLE_LIMIT) {
                    num_zero += 1;
                }
            }
        }

        // debug
        // println!("dir: {}, offset: {}, final: {}", dir, number, big_dial);

    }

    println!("We go through zero {} times", num_zero);
    // 7260 too high
    // 6088 too low! as is 6129
    // 6932 Hurrah! Brute force works!

    Ok(())
}