use std::error::Error;

use advent_utils::{get_config, Part};

mod game;

use self::game::GameState;

const N_PLAYERS: usize = 419;
const MAX_MARBLE_PART_ONE: i64 = 71052;
const MAX_MARBLE_PART_TWO: i64 = MAX_MARBLE_PART_ONE * 100;

fn main() -> Result<(), Box<Error>> {
    let config = get_config()?;

    match config.part {
        Part::One => {
            let high_score = GameState::produce(N_PLAYERS, MAX_MARBLE_PART_ONE)
                .high_score()
                .unwrap();

            println!("highest score will be {}", high_score);
        }
        Part::Two => {
            let high_score = GameState::produce(N_PLAYERS, MAX_MARBLE_PART_TWO)
                .high_score()
                .unwrap();

            println!("highest score will be {}", high_score);
        }
    }

    Ok(())
}
