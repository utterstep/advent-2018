use std::error::Error;

use advent_utils::{get_config, read_file, Part};

mod part_one;
mod part_two;
mod reactor;

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;

    let polymer = read_file(&config.input_file)?;
    let polymer = polymer.trim_end().chars();

    match config.part {
        Part::One => {
            println!(
                "Resulting polymer length is {}",
                part_one::perform_reactions(polymer).len()
            );
        }
        Part::Two => {
            let (bad, poly) = part_two::find_bad_unit(&polymer);

            println!(
                "Bad unit is {}, resulting polymer length is {}",
                bad,
                poly.len()
            );
        }
    }

    Ok(())
}
