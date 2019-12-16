use std::{
    error::Error,
    fs::File,
    io::{Read, Result as IOResult},
};

mod part_one;
mod part_two;
mod utils;

use advent_utils::{get_config, Config, Part};

fn get_problem_input(config: &Config) -> IOResult<String> {
    let mut f = File::open(&config.input_file)?;
    let mut raw_ids = String::with_capacity(f.metadata()?.len() as usize);

    f.read_to_string(&mut raw_ids)?;

    Ok(raw_ids)
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;
    let input = get_problem_input(&config)?;

    let ids = input.split_whitespace();

    match config.part {
        Part::One => {
            println!("checksum is {}", part_one::compute_checksum(ids));
        }
        Part::Two => {
            println!("valid id is {:?}", part_two::find_valid_id(ids));
        }
    }

    Ok(())
}
