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