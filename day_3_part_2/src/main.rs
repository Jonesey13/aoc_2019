use std::ops::AddAssign;
use std::{ops::Add, path::Path};
use std::fs;

fn main() {
    let text = fs::read_to_string(Path::new("data.txt")).expect("File not found!");

    let paths: Vec<Vec<&str>> = text.split("\n").map(|line| {line.split(",").collect()}).collect();
    let (path_1, path_2) = (&paths[0], &paths[1]);
 
    let path_1_squares: Vec<(Square, usize)> = build_path(&path_1);
    let path_2_squares: Vec<(Square, usize)> = build_path(&path_2);

    let intersection_dists: Vec<usize> = get_intersection(path_1_squares, path_2_squares);

    let closest_distance: usize = get_closest_to_origin(intersection_dists);

    println!("Closest Distance is {}", closest_distance)
}

fn build_path(path: &Vec<&str>) -> Vec<(Square, usize)> {
    let mut output = vec![];
    let mut current_square = Square::new(0, 0);
    let mut current_dist = 0;

    for instruction in path {
        let op_code = &instruction[..1];
        let number = instruction[1..instruction.len()].parse::<usize>().expect("Could Not Read Instruction!");

        let direction = match op_code {
            "R" => Square::new(1, 0),
            "L" => Square::new(-1, 0),
            "U" => Square::new(0, 1),
            "D" => Square::new(0, -1),
            _ => panic!("Unrecognised op code!")
        };

        for _ in 0..number {
            current_square += direction;
            current_dist += 1;
            output.push((current_square, current_dist));
        }
    }

    output
}

fn get_closest_to_origin(intersection_squares: Vec<usize>) -> usize {
    *intersection_squares.iter().min().expect("Empty Collection!")
}

fn get_intersection(path_1_squares: Vec<(Square, usize)>, path_2_squares: Vec<(Square, usize)>) -> Vec<usize> {
    let mut output = vec![];

    for (square1, dist1) in &path_1_squares {
        for (square2, dist2) in &path_2_squares  {
            if square1 == square2 {
                output.push(dist1 + dist2)
            }
        }
    }

    output
}

#[derive(Eq, PartialEq, Clone, Copy)]
struct Square {
    x: isize,
    y: isize
}

impl Square {
    fn new(x: isize, y: isize) -> Self { Self { x, y } }
}


impl Add for Square {
    type Output = Square;

    fn add(self, rhs: Self) -> Self::Output {
        Square::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Square {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs
    }
}