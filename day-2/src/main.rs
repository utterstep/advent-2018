use std::{
    error::Error,
    fs::File,
    io::{Read, Result as IOResult},
};

use serde::{de, de::Unexpected, Deserialize, Deserializer};
use serde_derive::Deserialize;

mod part_one;
mod part_two;
mod utils;

#[derive(Debug)]
enum Part {
    One,
    Two,
}

impl<'de> Deserialize<'de> for Part {
    fn deserialize<D>(deserializer: D) -> Result<Part, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input_value = String::deserialize(deserializer)?;

        match input_value.to_lowercase().as_ref() {
            "one" => Ok(Part::One),
            "two" => Ok(Part::Two),
            unknown => Err(de::Error::invalid_value(
                Unexpected::Str(unknown),
                &"one of [\"one\", \"two\"]",
            )),
        }
    }
}

#[derive(Deserialize, Debug)]
struct Config {
    part: Part,
    #[serde(default = "Config::default_input_file")]
    input_file: String,
}

impl Config {
    fn default_input_file() -> String {
        "input.txt".to_owned()
    }

    fn get_problem_input(&self) -> IOResult<String> {
        let mut f = File::open(&self.input_file)?;
        let mut raw_ids = String::with_capacity(f.metadata()?.len() as usize);

        f.read_to_string(&mut raw_ids)?;

        Ok(raw_ids)
    }
}

fn main() -> Result<(), Box<Error>> {
    let config = envy::prefixed("APP_").from_env::<Config>()?;
    let input = config.get_problem_input()?;

    let ids = input.split_whitespace();

    match config.part {
        Part::One => {
            println!("checksum is {}", part_one::compute_checksum(ids));
        },
        Part::Two => {
            println!("valid id is {:?}", part_two::find_valid_id(ids));
        },
    }

    Ok(())
}
