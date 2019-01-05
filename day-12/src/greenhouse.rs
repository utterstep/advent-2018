use std::{collections::VecDeque, str::FromStr};

use crate::{errors::greenhouse::GreenhouseParseError, plants::PlantState, rules::Rule};

pub(crate) struct GreenhouseState {
    state: VecDeque<PlantState>,
    rules: Vec<Rule>,
}

const INITIAL_STATE_PREFIX: &str = "initial state: ";

const DEAD_BORDER: usize = 4;

impl FromStr for GreenhouseState {
    type Err = GreenhouseParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s.lines();

        let state_data = input.next().ok_or(GreenhouseParseError::IncompleteData)?;

        if !state_data.starts_with(INITIAL_STATE_PREFIX) {
            return Err(state_data.into());
        }

        let state = state_data[INITIAL_STATE_PREFIX.len()..]
            .chars()
            .map(PlantState::from_char)
            .collect::<Result<VecDeque<_>, _>>()
            .map_err::<GreenhouseParseError, _>(|_| state_data.into())?;

        let mut rules = input
            .skip(1)
            .map(|s| s.parse::<Rule>())
            .collect::<Result<Vec<_>, _>>()?;

        rules.sort_unstable();

        Ok(Self { state, rules })
    }
}

impl GreenhouseState {
    pub fn growth_cycle(&mut self) {
        self.assure_borders();

        self.state
            .windows(5)
    }

    fn assure_borders(&mut self) {
        let left_dead = self
            .state
            .iter()
            .take(4)
            .take_while(|&&plant| plant == PlantState::Dead)
            .count();
        let right_dead = self
            .state
            .iter()
            .rev()
            .take(4)
            .take_while(|&&plant| plant == PlantState::Dead)
            .count();

        for _ in 0..(DEAD_BORDER - left_dead) {
            self.state.push_front(PlantState::Dead);
        }

        for _ in 0..(DEAD_BORDER - right_dead) {
            self.state.push_back(PlantState::Dead);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_greenhouse() {
        let data = include_str!("../short.txt");

        let greenhouse = data.parse::<GreenhouseState>().unwrap();
        assert_eq!(greenhouse.rules.len(), 14);
        assert_eq!(greenhouse.state.len(), 25);
    }

    #[test]
    fn test_assure_borders() {
        macro_rules! state_parse {
            ($data: expr) => {
                $data
                    .chars()
                    .map(PlantState::from_char)
                    .collect::<Result<VecDeque<_>, _>>()
                    .unwrap()
            };
        }

        let mut greenhouse = GreenhouseState {
            state: state_parse!("####...."),
            rules: Vec::new(),
        };

        greenhouse.assure_borders();
        assert_eq!(greenhouse.state, state_parse!("....####...."));

        let mut greenhouse = GreenhouseState {
            state: state_parse!("................####........."),
            rules: Vec::new(),
        };

        greenhouse.assure_borders();
        assert_eq!(greenhouse.state, state_parse!("................####........."));

        let mut greenhouse = GreenhouseState {
            state: state_parse!(".#"),
            rules: Vec::new(),
        };

        greenhouse.assure_borders();
        assert_eq!(greenhouse.state, state_parse!("....#...."));
    }
}
