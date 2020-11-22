use std::{path::Path, fs};

fn main() {
    let text = fs::read_to_string(Path::new("data.txt")).expect("File not found!");

    let lines = text.split("\n");

    let mut sum = 0;

    for line in lines {
        let mut number = line.parse::<i32>().expect("Not a number!");

        while number >= 0 {
            number = number / 3 - 2;
            sum += number.max(0);
        }
    }

    print!("{}", sum);
}

#[allow(dead_code)]
fn part_1() {
    let text = fs::read_to_string(Path::new("data.txt")).expect("File not found!");

    let lines = text.split("\n");

    let mut sum = 0;

    for line in lines {
        let number = line.parse::<u32>().expect("Not a number!");

        sum += number / 3 - 2;
    }

    print!("{}", sum);
}