use advent_of_code::utils::{InputHandler, Mode};

use crate::year_2023::day_1;

pub mod year_2023;

fn main() {
    let input_handler = InputHandler::new();

    day_1::part_2(&input_handler, Mode::Fake);
}
