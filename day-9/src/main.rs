use std::error::Error;

use serde::Deserialize;

use advent_utils::{get_custom_config, Part};

mod game;

use self::game::GameState;

#[derive(Deserialize)]
struct Config {
    part: Part,
    n_players: usize,
    max_marbles: i64,
}

fn main() -> Result<(), Box<Error>> {
    let config = get_custom_config::<Config>()?;

    let marbles_count = match config.part {
        Part::One => config.max_marbles,
        Part::Two => config.max_marbles * 100,
    };

    let high_score = GameState::produce(config.n_players, marbles_count)
        .high_score()
        .unwrap();

    println!("highest score will be {}", high_score);

    Ok(())
}
