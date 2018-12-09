use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use crate::claim::Claim;

fn read_file<P: AsRef<Path>>(p: P) -> Result<String, Box<Error>> {
    let file = File::open(p)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn from_file<P: AsRef<Path>>(p: P) -> Result<Vec<Claim>, Box<Error>> {
    let contents = read_file(p)?;

    Ok(contents
        .lines()
        .map(str::parse)
        .map(|r| r.unwrap())
        .collect())
}
