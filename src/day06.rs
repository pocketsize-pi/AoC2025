use std::fs::File;
use std::io::{prelude::*, BufReader};
use aoc_2025::*;

pub fn day06(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 06");

    let data = read_input(06, input_type, manual_name)?;

    #[derive(Debug)]
    enum OPERATION {
        Add,
        Mult,
    }

    let mut worksheet_num : Vec<Vec<u64>> = Vec::new();
    let mut worksheet_ops : Vec<OPERATION> = Vec::new();

    let num_ops = data[0].len();
    let num_value_rows = data.len()-2;
    let ops_line = data.len()-1;

    // initialise subvecs in the worksheet num
    for _i in 0..num_ops {
        worksheet_num.push(Vec::new());
    }

    for (r,row) in data.iter().enumerate() {
        // println!("{:?}", row);
        if r == ops_line {
            for value in row {
                if value == "+" {
                    worksheet_ops.push(OPERATION::Add);
                }
                else {
                    worksheet_ops.push(OPERATION::Mult);
                }
            }
        }

        else {
            // numbers
            for (n, num) in row.iter().enumerate() {
                worksheet_num[n].push(str_to_u64(num));
            }
        }
    }
    // println!("{:?}", worksheet_num);
    // println!("{:?}", worksheet_ops);

    let mut worksheet_total = 0;
    for (n, numbers) in worksheet_num.iter().enumerate() {
        // get operation and it's "null" value
        let op = &worksheet_ops[n];
        let mut line_total = match op {
            OPERATION::Add => {0}
            OPERATION::Mult => {1}
        };
        // operate over the numbers
        for num in numbers {
            match op {
                OPERATION::Add => { line_total += num }
                OPERATION::Mult => { line_total *= num }
            }
        }

        // tally up
        worksheet_total += line_total;
    }

    println!("The final total of the worksheet is {}", worksheet_total);
    // 4405895212738 yay!


    // re-read the data with all spaces for Part 2
    let file_name = match input_type {
        InputType::Sample => format!("data/day{:02}_sample.txt",06),
        InputType::Sample2 => format!("data/day{:02}_sample2.txt",06),
        InputType::Data=> format!("data/day{:02}_input.txt",06),
        InputType::Manual => format!("data/{}", manual_name),
    };
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut space_data = Vec::new();

    for line in reader.lines() {
        let line = line?;
        space_data.push(line);
    }
    // println!("{:?}", space_data);

    // reset value
    worksheet_total = 0;
    // the number of operations doesn't change, we can use that. But backwards
    for op in worksheet_ops.iter().rev() {

        // println!("---");
        // println!("op: {:?}", op);

        let mut line_total = match op {
            OPERATION::Add => {0}
            OPERATION::Mult => {1}
        };

        let mut numbers = Vec::new();
        while !space_data[0].is_empty() {
            let mut this_value: String = Default::default();
            for i in 0..num_value_rows+1 {
                // println!("   build: {:?}", space_data[i].pop().unwrap());
                this_value.push(space_data[i].pop().unwrap());
            }
            // println!("this_value: {:?}", this_value);
            this_value = this_value.trim().parse().unwrap();
            // println!("this_value: {:?}", this_value);

            // check if we are done (all spaces), or end of line!
            if (this_value == "" )| space_data[0].is_empty(){

                // but we mustn't forget the last one!
                if space_data[0].is_empty() {
                    numbers.push(str_to_u64(&*this_value));
                }

                // when all are spaces and we remove them, we have all the numbers for this op
                for num in numbers {
                    match op {
                        OPERATION::Add => { line_total += num }
                        OPERATION::Mult => { line_total *= num }
                    }
                }
                // println!("line_total {:?}", line_total);
                worksheet_total += line_total;
                break;
            }
            else {
                // convert and store number
                numbers.push(str_to_u64(&*this_value));
            }
        }

    }

    println!("The cephalopod total of the worksheet is {}", worksheet_total);
    //7450962480322 too low
    // 7450962489289 we get the right answer when we don't forget the last one!

    Ok(())
}