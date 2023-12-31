use advent_of_code::utils::{InputHandler, Mode};

use crate::year_2023::day_1;
use crate::year_2023::day_2;
use crate::year_2023::day_3;

pub mod year_2023;

fn main() {
    let input_handler = InputHandler::new();

    day_3::part_2(&input_handler, Mode::Real);
}
