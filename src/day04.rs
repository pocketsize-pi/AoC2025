use aoc_2025::*;

pub fn day04(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 04");


    let data = read_input(04, input_type, manual_name)?;
    let mut paper_room = Vec::new();
    let max_x = data[0][0].len() as u32;
    let max_y = data.len() as u32;

    for (r, row) in data.iter().enumerate() {
        // println!("{:?}", row);
        let row_chars = str_to_chars(&row[0]);
        for (c, col) in row_chars.iter().enumerate() {
            if *col == '@' {
                paper_room.push(Point{r_y: r as i32, c_x:c as i32});
            }
        }
    }

    let mut movable_rolls = 0;

    for location in &paper_room {
        let mut num_rolls = 0;
        let neighbours = location.get_neighbours(max_x, max_y);
        // println!("{:?}", location);
        // println!("  {:?}", neighbours);
        for neighbour in &neighbours {
            if paper_room.contains(&neighbour) {
                num_rolls += 1;
            }
        }
        if num_rolls < 4 {
            movable_rolls += 1;
            // println!("valid {:?}", location);
            // println!("  {:?}", neighbours);
        }
    }

    println!("We can move {} toilet rolls", movable_rolls);
    // 1527 is the right answer


    Ok(())
}