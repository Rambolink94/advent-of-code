use std::collections::HashMap;

use advent_of_code::utils::{InputHandler, Mode};

pub fn part_1(input_handler: &InputHandler, mode: Mode) {
    let lines = input_handler.parse_lines("./src/year_2023/day_1/input.txt", mode);
    
    let mut total = 0;
    for line in lines {
        if let Ok(line) = line {
            let mut calibration_value = String::new();
            let mut last_digit = 'A';
            for char in line.chars() {
                if char.is_ascii_digit() {
                    if !last_digit.is_ascii_digit() {
                        calibration_value.push(char);
                    }
                    
                    last_digit = char;
                }
            }
            
            calibration_value.push(last_digit);
            println!("{} - {}", line, calibration_value);
            let value = calibration_value.parse::<i32>().unwrap();
            total += value;
        }
    }

    println!("\nTotal: {total}");
}

pub fn part_2(input_handler: &InputHandler, mode: Mode) {
    let lines = input_handler.parse_lines("./src/year_2023/day_1/input.txt", mode);
    
    let mut total = 0;
    for line in lines {
        if let Ok(line) = line {
            let indexes = contains_numbers(&line);
            println!("{:?}", indexes);

            let mut min_max = [0, 0];
            let mut min_set = false;

            let mut index = 0;
            for char in line.chars() {
                if char.is_ascii_digit() {
                    println!("Found number: {char}");
                    if !min_set {
                        min_max[0] = index;
                        min_set = true;
                    }
                    else {
                        min_max[1] = index;
                    }
                }

                index += 1;
            }
            
            if min_max[0] > min_max[1] {
                min_max[1] = min_max[0];
            }

            let mut calibration_value = String::new();
            let length = indexes.len();
            let mut val = line.chars().nth(min_max[0]).unwrap();
            if length > 0 && indexes[0].0 <= min_max[0] {
                val = indexes[0].1;
            }

            calibration_value.push(val);

            val = line.chars().nth(min_max[1]).unwrap();
            if length > 0 && indexes[length - 1].0 > min_max[1] {
                val = indexes[length - 1].1;
            }

            calibration_value.push(val);

            println!("{} - {}\n", line, calibration_value);
            let value = calibration_value.parse::<i32>().unwrap();
            total += value;
        }
    }

    println!("\nTotal: {total}");
}

fn contains_numbers(line: &String) -> Vec<(usize, char)> {
    let numbers = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'), 
        ("eight", '8'),
        ("nine", '9'),
    ]);
    
    let mut indexes = Vec::new();
    for (key, value) in numbers {
        let index = line.find(key);
        if let Some(index) = index {
            indexes.push((index, value));
        }

        let index = line.rfind(key);
        if let Some(index) = index {
            indexes.push((index, value));
        }
    }

    indexes.sort_by_key(|x| x.0);

    indexes
}