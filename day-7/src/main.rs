use serde_derive::Deserialize;
use std::{error::Error, num::NonZeroUsize};

use advent_utils::{get_custom_config, Part};

mod graph;
mod instruction;
mod parser;
mod workers;

mod traversers;

use self::parser::parse_graph;
use self::traversers::{PooledTraverser, SimpleTraverser};

#[derive(Debug, Deserialize)]
struct Config {
    input_file: String,
    part: Part,
    work_price_delta: Option<i32>,
    workers: Option<NonZeroUsize>,
}

fn main() -> Result<(), Box<Error>> {
    let c = get_custom_config::<Config>()?;
    let graph = parse_graph(c.input_file);

    match c.part {
        Part::One => {
            let traverser = SimpleTraverser::from(graph);

            println!("Suggested work order is: {}", traverser.collect::<String>());
        }
        Part::Two => {
            let delta = c.work_price_delta.unwrap();
            let workers = c.workers.or_else(|| NonZeroUsize::new(2)).unwrap();

            let traverser = PooledTraverser::new(graph, delta, workers);

            println!(
                "work will be all done at: {}",
                traverser.graph_finish_time()
            )
        }
    }

    Ok(())
}
