#[macro_use]
extern crate nom;

mod log;
mod stats;

use advent_utils::parse_file;

use crate::stats::{GuardStats, WorstSelectionStrategy};

fn main() -> Result<(), Box<std::error::Error>> {
    let mut log = parse_file("full.txt")?;

    log.sort_unstable();

    let mut stats = GuardStats::new();
    stats.process_log(log.iter());

    let winner_one = stats
        .get_worst_guard(WorstSelectionStrategy::Total)
        .unwrap();

    let winner_two = stats
        .get_worst_guard(WorstSelectionStrategy::Frequency)
        .unwrap();

    println!(
        "Worst guard by first strategy is #{}, his worst time is {:?}",
        winner_one.id,
        winner_one.worst_minute(),
    );

    println!(
        "Worst guard by second strategy is #{}, his worst time is {:?}",
        winner_two.id,
        winner_two.worst_minute(),
    );

    Ok(())
}
