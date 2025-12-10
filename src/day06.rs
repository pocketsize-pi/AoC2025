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


    Ok(())
}