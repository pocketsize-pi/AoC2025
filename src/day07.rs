use std::collections::{HashMap, VecDeque};
use aoc_2025::*;
use aoc_2025::UdlrDirection::{Down, Left, Right};

pub fn day07(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 07");


    let data = read_input(07, input_type, manual_name)?;

    let mut gates_map = HashMap::new();
    let mut start_point  = Point::default();
    let map_rows = data.len();
    let map_cols = data[0][0].len();

    for i  in 0..data.len() {
        // let row = &data[i];
        let my_row = str_to_chars(&data[i][0]);

        for j in 0..my_row.len() {
            if my_row[j] == 'S' {
                start_point = Point{r_y: i as i32, c_x:j as i32}
            }
            else if my_row[j] == '^' {
                gates_map.insert(Point{r_y: i as i32, c_x:j as i32}, my_row[j]);
            }

        }
    }

    let mut tachion_queue = VecDeque::new();
    let mut num_splits = 0;
    let mut tachion_visited = VecDeque::new();

    tachion_queue.push_front(start_point);

    while !tachion_queue.is_empty() {

        // pop tachion position
        let mut this_tachion = tachion_queue.pop_front().unwrap();
        // println!("this tachion {:?}", this_tachion);

        // add it to the visited list (we have already checked it is not visited)
        tachion_visited.push_back(this_tachion);

        // move it down
        this_tachion.move_one_udlr(&Down);

        // check that we are still inside the map!
        if this_tachion.within_dimensions(map_cols as u32, map_rows as u32) {
            // is it a splitter?
            if gates_map.contains_key(&this_tachion) {
                // if it is a splitter
                // increase splitter count
                num_splits += 1;
                // create two points, left and right
                let tachion_left = this_tachion.move_one_to_new_udlr(Left);
                let tachion_right = this_tachion.move_one_to_new_udlr(Right);

                // if we haven't already been here, then add them to the queue
                // if they are within limits!
                if tachion_left.within_dimensions(map_cols as u32, map_rows as u32) &
                    !tachion_visited.contains(&tachion_left) {
                    tachion_queue.push_front(tachion_left);
                }
                if tachion_right.within_dimensions(map_cols as u32, map_rows as u32) &
                    !tachion_visited.contains(&tachion_right) {
                    tachion_queue.push_front(tachion_right);
                }

            }
            else {
                // if not, check if we have already been here, and if not, add it to the queue
                // we have already moved this tachion, so it is in the new position
                if !tachion_visited.contains(&this_tachion) {
                    tachion_queue.push_front(this_tachion);
                }
            }
        }

    }

    println!("There are {num_splits} splits of the tachion beam");
    // 1592

    Ok(())
}