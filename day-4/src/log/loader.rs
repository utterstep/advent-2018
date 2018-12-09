use std::{
    collections::BTreeSet,
    error::Error,
    fs::File,
    io::{Read, BufReader},
    path::Path,
};

use crate::log::entry::LogEntry;

fn read_file<P: AsRef<Path>>(p: P) -> Result<String, Box<Error>> {
    let file = File::open(p)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn from_file<P: AsRef<Path>>(p: P) -> Result<BTreeSet<LogEntry>, Box<Error>> {
    let contents = read_file(p)?;

    Ok(contents
        .lines()
        .map(str::parse)
        .map(|r| r.unwrap())
        .collect())
}
