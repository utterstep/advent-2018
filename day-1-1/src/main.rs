use std::{error::Error, fs::File, io::prelude::*};

const INPUT_FILE: &str = "input.txt";

fn main() -> Result<(), Box<Error>> {
    let mut f = File::open(INPUT_FILE)?;
    let mut buf = String::with_capacity(f.metadata()?.len() as usize);

    f.read_to_string(&mut buf)?;

    let res = buf
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .sum::<i64>();

    println!("resulting frequency is: {}", res);

    Ok(())
}
