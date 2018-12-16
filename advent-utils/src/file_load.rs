use std::{
    error::Error,
    fmt::Debug,
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};

pub fn read_file<P: AsRef<Path>>(p: P) -> Result<String, Box<Error>> {
    let file = File::open(p)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn parse_file<P, T>(p: P) -> Result<Vec<T>, Box<Error>>
where
    P: AsRef<Path>,
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let contents = read_file(p)?;

    Ok(contents
        .lines()
        .map(str::parse)
        .map(|r| r.unwrap())
        .collect())
}
