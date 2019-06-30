use std::error::Error;

use advent_utils::{get_custom_config, parse_file, Part};

use serde::Deserialize;

mod sky;
mod star;

use self::sky::Sky;

#[derive(Deserialize)]
struct Config {
    part: Part,
    input_file: String,
    #[serde(default)]
    visualize: bool,
}

fn main() -> Result<(), Box<Error>> {
    let config = get_custom_config::<Config>()?;
    let stars = parse_file(config.input_file)?;
    let mut sky = Sky::new(stars);

    match config.part {
        Part::One => {
            sky.advance_to_message(config.visualize);

            println!("{}", sky);
        }
        Part::Two => {
            let steps = sky.advance_to_message(config.visualize);

            println!("resulting message:\n\n{}\n", sky);
            println!("appeared after {} seconds", steps);
        }
    }

    Ok(())
}
