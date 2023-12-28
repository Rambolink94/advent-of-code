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

    let lines = input_handler.parse_lines("./src/year_2023/day_1/input.txt", mode);
    
    let mut total = 0;
    for line in lines {
        if let Ok(line) = line {
            let mut min_max = [0, 0];
            let mut min_set = false;

            let mut index = 0;
            let mut word_start: Option<usize> = None;
            for char in line.chars() {
                let indexes = contains_numbers(&line);
                println!("{:?}", indexes);
                print!("{char}");
                if char.is_ascii_digit() {
                    println!("\nFound number: {char}");
                    if !min_set {
                        min_max[0] = index;
                        min_set = true;
                    }
                    else {
                        min_max[1] = index;
                    }

                    word_start = None;
                }
                else {
                    if word_start.is_none() {
                        word_start = Some(index);
                    }
                    
                    let mut reset = false;
                    if let Some(word_start) = word_start {
                        let word = &line[word_start..index + 1];
                        if numbers.contains_key(word) {
                            println!("\nFound word: {word}");
                            if !min_set {
                                min_max[0] = word_start;
                                min_set = true;
                            }
                            else {
                                min_max[1] = word_start;
                            }

                            reset = true;
                        }
                    }

                    if reset {
                        word_start = None;
                    }
                }

                index += 1;
            }
            
            println!("{:?}", min_max);
            let mut calibration_value = String::new();
            for i in min_max.iter() {
                let current = line.chars().nth(*i).unwrap();
                if current.is_ascii_digit() {
                    calibration_value.push(current);
                } else {
                    for j in *i..line.len() {
                        let word = &line[*i..j + 1];
                        if numbers.contains_key(word) {
                            println!("Pushing {word}..");
                            calibration_value.push(numbers[word]);
                        }
                    }
                }
            }

            println!("{} - {}", line, calibration_value);
            let value = calibration_value.parse::<i32>().unwrap();
            total += value;
        }
    }

    println!("\nTotal: {total}");
}

fn contains_numbers(word: &String) -> Vec<(usize, char)> {
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
        let index = word.find(key);
        if let Some(index) = index {
            indexes.push((index, value));
        }
    }

    indexes.sort_by_key(|x| x.0);

    indexes
}