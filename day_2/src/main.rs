use std::{path::Path, fs};

fn main() {
    let text = fs::read_to_string(Path::new("data.txt")).expect("File not found!");

    let numbers: Vec<usize> = text.split(",").map(|num| {num.parse::<usize>().expect("Not a number!")}).collect();

    for i in 0..99 {
        for j in 0..99 {
            let output = run_program(&numbers, i, j);

            if output == 19690720 {
                println!("{} {}", i, j);
                return;
            }
        } 
    }
}

fn run_program(original_numbers: &Vec<usize>, input_1: usize, input_2: usize) -> usize {
    let mut numbers = original_numbers.clone();

    numbers[1] = input_1;
    numbers[2] = input_2;
    
    let mut read_pos = 0;

    while read_pos < numbers.len() {
        let op_code = numbers[read_pos];

        if op_code == 1 {
            let add_pos_1 = numbers[read_pos + 1];
            let add_pos_2 =  numbers[read_pos + 2];
            let add_value = numbers[add_pos_1] + numbers[add_pos_2];
            let write_pos = numbers[read_pos + 3];
            numbers[write_pos] = add_value;
            read_pos += 4;
        } else if op_code == 2 {
            let mul_pos_1 = numbers[read_pos + 1];
            let mul_pos_2 =  numbers[read_pos + 2];
            let mul_value = numbers[mul_pos_1] * numbers[mul_pos_2];
            let write_pos = numbers[read_pos + 3];
            numbers[write_pos] = mul_value;
            read_pos += 4;
        } else if op_code != 99 {
            panic!("Wrong op code at pos {}", read_pos);
        } else {
            break;
        }
    }

    return numbers[0];
}
