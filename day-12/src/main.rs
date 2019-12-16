use std::error::Error;

use advent_utils::{get_config, read_file, Part};

mod errors;
// mod greenhouse;
mod plants;
mod rules;

// use self::greenhouse::GreenhouseState;

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;
    let data = read_file(config.input_file)?;
    // let greenhouse: GreenhouseState = data.parse()?;

    Ok(())
}
