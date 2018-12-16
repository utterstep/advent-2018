use std::{collections::HashSet, error::Error, fs::File, io::prelude::*, path::Path};

const INPUT_FILE: &str = "input.txt";

fn get_data<P: AsRef<Path>>(file_name: P) -> std::io::Result<String> {
    let mut f = File::open(file_name)?;
    let mut data = String::with_capacity(f.metadata()?.len() as usize);

    f.read_to_string(&mut data)?;

    Ok(data)
}

struct RepeatChecker<T> {
    inner_set: HashSet<T>,
}

impl<T> RepeatChecker<T>
where
    T: Eq + std::hash::Hash + Copy,
{
    fn new() -> Self {
        Self {
            inner_set: HashSet::new(),
        }
    }

    fn check(&mut self, value: T) -> Option<T> {
        if self.inner_set.insert(value) {
            None
        } else {
            Some(value)
        }
    }
}

fn main() -> Result<(), Box<Error>> {
    let data = get_data(INPUT_FILE)?;
    let data_cycle = data
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .cycle();

    let mut current: i64 = 0;
    let mut repetition_checker = RepeatChecker::new();

    for value in data_cycle {
        if let Some(repeated) = repetition_checker.check(current) {
            println!("repeated at frequency: {}", repeated);
            break;
        }

        current += value;
    }

    Ok(())
}
