use std::collections::{HashSet, HashMap};

use advent_of_code::utils::{InputHandler, Mode};

/// Determines the sum of all valid games
pub fn part_1(input_handler: &InputHandler, mode: Mode) {
    let lines = input_handler.parse_lines("./src/year_2023/day_2/input.txt", mode);

    let valid_games = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let mut possible_games: HashSet<i32> = HashSet::new();
    for line in lines {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split(':').collect();
            let game_id: i32 = parts[0][5..].parse().unwrap();
            
            let mut is_valid = true;
            let games = parts[1].split(';');
            for game in games {
                let checks = game.split(',');
                for check in checks {
                    let pair: Vec<&str> = check.split_whitespace().collect();
                    let value: i32 = pair[0].parse().unwrap();
                    if value > valid_games[pair[1]] {
                        is_valid = false;
                        break;
                    }
                }
            }

            if is_valid {
                possible_games.insert(game_id);
                println!("Valid: {game_id}");
            }
        }
    }
    
    let id_sum: i32 = possible_games.iter().sum();

    println!("The sum of valid game ids is {id_sum}.");
}

/// Determines the fewest number of cubes of each color per game then adds their power (red + green + blue).
pub fn part_2(input_handler: &InputHandler, mode: Mode) {
    let lines = input_handler.parse_lines("./src/year_2023/day_2/input.txt", mode);
    
    let mut total = 0;
    for line in lines {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split(':').collect();
            
            let mut game_map = HashMap::new();

            let games = parts[1].split(';');
            for game in games {
                let checks = game.split(',');
                for check in checks {
                    let pair: Vec<&str> = check.split_whitespace().collect();
                    let key = pair[1];
                    let value: i32 = pair[0].parse().unwrap();

                    let entry = game_map.entry(key).or_insert(0);
                    if value > *entry {
                        *entry = value;
                    }
                }
            }

            println!("{:?}", game_map);
            total += game_map.iter().map(|(_, &v)| v).product::<i32>();
        }
    }

    println!("The sum of game powers is {total}.");
}