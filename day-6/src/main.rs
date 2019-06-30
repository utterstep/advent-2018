use std::error::Error;

use serde::Deserialize;

use advent_utils::{get_custom_config, parse_file, Part};

use day_6_parser::Point;

mod world;

use crate::world::World;

#[derive(Debug, Deserialize)]
struct Config {
    part: Part,
    input_file: String,
    max_distance: Option<i32>,
}

fn main() -> Result<(), Box<Error>> {
    let config = get_custom_config::<Config>()?;
    let points: Vec<Point> = parse_file(&config.input_file)?;
    let world = World::from_points(points);

    match config.part {
        Part::One => {
            println!(
                "Largest closed area has size of {}",
                world.largest_finite_area().unwrap()
            );
        }
        Part::Two => {
            let max_distance = config
                .max_distance
                .expect("Max distance should be specified for part two");

            println!(
                "Safe area has size of {}",
                world.safe_area_size(max_distance),
            )
        }
    };

    Ok(())
}
