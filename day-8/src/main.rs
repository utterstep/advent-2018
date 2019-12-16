use std::error::Error;

use advent_utils::{get_config, read_file, Part};

mod node;

use self::node::Node;

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;
    let input_data = read_file(config.input_file)?;
    let mut node = Node::from_iter(&mut input_data.split_whitespace().map(|n| n.parse().unwrap()))?;

    match config.part {
        Part::One => {
            println!("metadata sum is: {}", node.checksum());
        }
        Part::Two => {
            println!("node value is: {}", node.value());
        }
    }

    Ok(())
}
