use std::collections::HashSet;

use advent_of_code::utils::{InputHandler, Mode};

pub fn part_1(input_handler: &InputHandler, mode: Mode) {
    let lines: Result<Vec<Vec<char>>, std::io::Error> = input_handler
        .parse_lines("./src/year_2023/day_3/input.txt", mode)
        .map(|r| {
            r.map(|s| s.chars().collect())
        })
        .collect();

    let lines = match lines {
        Ok(value) => value,
        Err(error) => panic!("Error converting to vector of chars: {error}"),
    };
    
    let mut total = 0;
    let mut i = 0;
    while i < lines.len() {
        let mut j = 0;
        let line = &lines[i];
        while j < lines[i].len() {
            let mut num_string = String::new();
            let mut is_neighbour = false;
            while j < lines[i].len() && line[j].is_numeric() {
                num_string.push(line[j]);
                
                // Check right first, since if it is engine, we are done with this number
                if check_neighbour(&lines, Some(i), Some(j + 1)) {
                    is_neighbour = true;
                    j += 1;
                    
                    break;
                }
                
                is_neighbour |= check_neighbour(&lines, Some(i + 1), Some(j + 1));  // br
                is_neighbour |= check_neighbour(&lines, Some(i + 1), Some(j));  // bm
                is_neighbour |= check_neighbour(&lines, Some(i + 1), j.checked_sub(1)); // bl
                is_neighbour |= check_neighbour(&lines, Some(i), j.checked_sub(1));  // ml
                is_neighbour |= check_neighbour(&lines, i.checked_sub(1), j.checked_sub(1));  // tl
                is_neighbour |= check_neighbour(&lines, i.checked_sub(1), Some(j)); // tm
                is_neighbour |= check_neighbour(&lines, i.checked_sub(1), Some(j + 1)); // tr

                j += 1;
            }

            if is_neighbour {
                println!("Found engine part {num_string}");
                total += num_string.parse::<i32>().unwrap();
            }

            j += 1;
        }

        i += 1;
    }

    println!("Total part numbers: {total}");
}

pub fn part_2(input_handler: &InputHandler, mode: Mode) {
    let lines: Result<Vec<Vec<char>>, std::io::Error> = input_handler
        .parse_lines("./src/year_2023/day_3/input.txt", mode)
        .map(|r| {
            r.map(|s| s.chars().collect())
        })
        .collect();

    let lines = match lines {
        Ok(value) => value,
        Err(error) => panic!("Error converting to vector of chars: {error}"),
    };

    let mut total = 0;
    let mut i = 0;
    while i < lines.len() {
        let mut j = 0;
        while j < lines[i].len() {
            if lines[i][j] == '*' {
                // check neighbours
                let mut x = 0;
                let mut found_nums = HashSet::new();
                while x <= 2 {
                    let mut y = 0;
                    while y <= 2 {
                        let number = try_get_number(&lines, (i + 1).checked_sub(x), (j + 1).checked_sub(y));
                        if let Some(number) = number {
                            found_nums.insert(number);
                        }

                        y += 1;
                    }

                    x += 1;
                }

                if found_nums.len() == 2 {
                    println!("{:?}", found_nums);
                    let gear_ratio: i32 = found_nums.into_iter().product();
                    println!("Found gear ratio of {gear_ratio} at ({i}, {j})");

                    total += gear_ratio;
                }
            }

            j += 1;
        }

        i += 1;
    }

    println!("Total part numbers: {total}");
}

fn check_neighbour(lines: &Vec<Vec<char>>, i: Option<usize>, j: Option<usize>) -> bool {
    if let (Some(i), Some(j)) = (i, j) {
        if i < lines.len() && j < lines[i].len() {
            let c = lines[i][j];
            return c.is_ascii_punctuation() && c != '.'
        }
    }

    false
}

fn try_get_number(lines: &Vec<Vec<char>>, i: Option<usize>, j: Option<usize>) -> Option<i32> {
    if let (Some(i), Some(j)) = (i, j) {
        let mut num_string = String::new();
        // TODO: Consolidate these two loops and reduce repeated code.
        if i < lines.len() && j < lines[i].len() {
            if !lines[i][j].is_numeric() {
                return None;
            }

            let mut x = 0;
            while let Some(n) = j.checked_sub(x) {
                let current_char = lines[i][n];
                if !current_char.is_numeric() {
                    break;
                }

                num_string.insert(0, current_char);
                x += 1;
            }
            
            x = 1;
            while let Some(n) = j.checked_add(x) {
                if n >= lines[i].len() {
                    // On edge of line
                    break;
                }

                let current_char = lines[i][n];
                if !current_char.is_numeric() {
                    break;
                }

                num_string.push(current_char);
                x += 1;
            }

            let found_num: i32 = match num_string.parse() {
                Ok(num) => num,
                Err(error) => panic!("Could not parse {num_string}: {error}"),
            };
    
            return Some(found_num);
        }
    }

    None
}