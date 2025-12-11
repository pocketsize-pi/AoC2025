use std::fs::File;
use std::io::{prelude::*, BufReader};

// INPUT PROCESSING
#[derive(PartialEq, Clone, Copy)]
pub enum InputType {
    Sample,
    Sample2,
    Data,
    Manual,
}

pub fn read_input(day: u8, input: InputType, manual_name: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {

    let file_name = match input {
        InputType::Sample => format!("data/day{:02}_sample.txt",day),
        InputType::Sample2 => format!("data/day{:02}_sample2.txt",day),
        InputType::Data=> format!("data/day{:02}_input.txt",day),
        InputType::Manual => format!("data/{}", manual_name),
    };
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let words: Vec<String> = line.split_whitespace()
            .map(|s| s.to_string())
            .collect();

        data.push(words);
    }

    // println!("{:?}", data);
    Ok(data)
}

// String conversions
pub fn str_to_chars(input: &str) -> Vec<char> {
    input.chars().collect()
}

pub fn str_to_i32 (input: &str) -> i32 {
    input.parse::<i32>().unwrap()
}

pub fn str_to_i64 (input: &str) -> i64 {
    input.parse::<i64>().unwrap()
}

pub fn str_to_u32 (input: &str) -> u32 {
    input.parse::<u32>().unwrap()
}

pub fn str_to_u64 (input: &str) -> u64 {
    input.parse::<u64>().unwrap()
}




pub const NORTH: Point = Point { r_y:-1, c_x:0};
pub const SOUTH: Point = Point { r_y:1, c_x:0};
pub const EAST: Point = Point { c_x:1, r_y:0};
pub const WEST: Point = Point { c_x:-1, r_y:0};
pub const STAY: Point = Point { c_x:0, r_y:0};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

pub const DIRECTION_LIST : [Direction; 8] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
    Direction::NorthEast,
    Direction::NorthWest,
    Direction::SouthEast,
    Direction::SouthWest];

#[derive(Clone, Copy, PartialEq)]
pub enum UdlrDirection {
    Up,
    Down,
    Left,
    Right,
}

pub fn get_direction (udlr_dir: &UdlrDirection) -> Direction {
    match udlr_dir {
        UdlrDirection::Up => {Direction::North}
        UdlrDirection::Down => {Direction::South}
        UdlrDirection::Left => {Direction::West}
        UdlrDirection::Right => {Direction::East}
    }
}


// COORDINATES
// column is x, row is y
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Default)]
pub struct Point {
    pub c_x: i32,
    pub r_y: i32,
}

impl Point {

    pub fn new(c_x: usize, r_y: usize) -> Self {
        Point{c_x: c_x as i32, r_y: r_y as i32}
    }
    pub fn default() -> Self {
        Point{c_x: 0, r_y: 0}
    }
    pub fn move_one(&mut self, direction: &Direction)
    {
        self.move_along(direction, &1);
    }
    pub fn move_along(&mut self, direction: &Direction, length: &i32)
    {
        match direction {
            Direction::North => self.r_y -= length,
            Direction::South => self.r_y += length,
            Direction::East => self.c_x += length,
            Direction::West => self.c_x -= length,
            Direction::NorthEast => {
                self.r_y -= 1;
                self.c_x += 1;
            }
            Direction::NorthWest => {
                self.r_y -= 1;
                self.c_x -= 1;
            }
            Direction::SouthEast => {
                self.r_y += 1;
                self.c_x += 1;
            }
            Direction::SouthWest => {
                self.r_y += 1;
                self.c_x -= 1;
            }
        }
    }

    pub fn move_one_udlr(&mut self, direction: &UdlrDirection) {
        self.move_along(&get_direction(direction), &1);
    }

    pub fn move_along_udlr(&mut self, direction: &UdlrDirection, length: &i32)
    {
        self.move_along(&get_direction(direction), length);
    }

    pub fn within_dimensions(self, max_c_x: u32, max_r_y: u32) -> bool{
        (self.r_y >= 0) & (self.r_y < max_r_y as i32) & (self.c_x >= 0) & (self.c_x < max_c_x as i32)
    }

    pub fn add(&mut self, offset_point: Point) {
        self.c_x += offset_point.c_x;
        self.r_y += offset_point.r_y;
    }

    pub fn add_to_new(self, offset_point: Point) -> Point {
        Point{c_x: self.c_x + offset_point.c_x, r_y: self.r_y + offset_point.r_y}
    }

    pub fn move_one_to_new(self, direction: &Direction) -> Point
    {
        let mut output = self.clone();
        match direction {
            Direction::North => output.r_y -= 1,
            Direction::South => output.r_y += 1,
            Direction::East => output.c_x += 1,
            Direction::West => output.c_x -= 1,
            Direction::NorthEast => {
                output.r_y -= 1;
                output.c_x += 1;
            }
            Direction::NorthWest => {
                output.r_y -= 1;
                output.c_x -= 1;
            }
            Direction::SouthEast => {
                output.r_y += 1;
                output.c_x += 1;
            }
            Direction::SouthWest => {
                output.r_y += 1;
                output.c_x -= 1;
            }
        }
        output
    }

    pub fn move_one_to_new_udlr(self, udlr_direction: UdlrDirection) -> Point {
        self.move_one_to_new(&get_direction(&udlr_direction))
    }

    pub fn get_neighbours(self, max_x: u32, max_y: u32) -> Vec<Point> {
        let mut neighbours :Vec<Point> = Vec::new();
        for dir in DIRECTION_LIST {
            let new_point =  self.move_one_to_new(&dir);
            if new_point.within_dimensions(max_x, max_y) {
                neighbours.push(new_point);
            }
        }
        neighbours
    }

}

