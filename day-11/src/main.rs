use std::error::Error;

use serde::Deserialize;

use advent_utils::{get_custom_config, Part};

mod grid;

use crate::grid::Grid;

#[derive(Debug, Deserialize)]
struct Config {
    part: Part,
    serial: i64,
}

const GRID_SIZE: usize = 300;

const PART_ONE_QUADRANT_SIDE: usize = 3;

fn main() -> Result<(), Box<Error>> {
    let config = get_custom_config::<Config>()?;
    let grid = Grid::new(GRID_SIZE, config.serial);

    match config.part {
        Part::One => {
            let (coords, power) = grid.find_maximum(PART_ONE_QUADRANT_SIDE).unwrap();

            println!("Max quadrant is at {:?}, with power sum: {}", coords, power);
        }
        Part::Two => {
            let (side, (coords, power)) = grid.find_global_maximum().unwrap();

            println!("Max power quadrant has side of {}", side);
            println!("It is located at {:?}, with power sum: {}", coords, power);
        }
    }

    Ok(())
}
