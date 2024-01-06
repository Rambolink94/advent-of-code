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
    let _lines = input_handler.parse_lines("./src/year_2023/day_3/input.txt", mode);
    
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