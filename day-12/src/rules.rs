use std::str::FromStr;

use crate::{errors::rules::RuleParseError, plants::PlantState};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Rule {
    pattern: [PlantState; 5],
    outcome: PlantState,
}

impl Rule {
    pub fn apply(&self, data: &[PlantState]) -> Option<PlantState> {
        debug_assert_eq!(data.len(), 5);

        if self.pattern == data {
            Some(self.outcome)
        } else {
            None
        }
    }
}

impl FromStr for Rule {
    type Err = RuleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split(" => ");

        let pattern_data = splitted
            .next()
            .ok_or_else(|| RuleParseError::WrongRuleFormat(s.to_owned()))?;
        let pattern_data: Result<Vec<_>, _> = pattern_data
            .chars()
            .take(5)
            .map(PlantState::from_char)
            .collect();
        let pattern_vec = pattern_data?;

        if pattern_vec.len() != 5 {
            return Err(s.into());
        };

        // FIXME: there shold be more elegant solution...
        let pattern: [PlantState; 5] = [
            pattern_vec[0],
            pattern_vec[1],
            pattern_vec[2],
            pattern_vec[3],
            pattern_vec[4],
        ];

        let outcome_data = splitted
            .next()
            .ok_or_else::<RuleParseError, _>(|| s.into())?;
        let outcome_char = outcome_data
            .chars()
            .next()
            .ok_or_else::<RuleParseError, _>(|| s.into())?;
        let outcome = PlantState::from_char(outcome_char)?;

        Ok(Self { pattern, outcome })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_parse() {
        // correct cases
        let data: Result<Rule, _> = "...#. => .".parse();
        assert!(data.is_ok());

        let rule = data.unwrap();
        assert_eq!(rule.outcome, PlantState::Dead);
        assert_eq!(
            rule.pattern,
            [
                PlantState::Dead,
                PlantState::Dead,
                PlantState::Dead,
                PlantState::Alive,
                PlantState::Dead,
            ]
        );

        let data: Result<Rule, _> = "#.#.# => #".parse();
        assert!(data.is_ok());

        let rule = data.unwrap();
        assert_eq!(rule.outcome, PlantState::Alive);
        assert_eq!(
            rule.pattern,
            [
                PlantState::Alive,
                PlantState::Dead,
                PlantState::Alive,
                PlantState::Dead,
                PlantState::Alive,
            ]
        );

        let data: Result<Rule, _> = "####. => .".parse();
        assert!(data.is_ok());

        let rule = data.unwrap();
        assert_eq!(rule.outcome, PlantState::Dead);
        assert_eq!(
            rule.pattern,
            [
                PlantState::Alive,
                PlantState::Alive,
                PlantState::Alive,
                PlantState::Alive,
                PlantState::Dead,
            ]
        );

        // incorrect cases
        assert!("12345 => #".parse::<Rule>().is_err());
        assert!("###. => #".parse::<Rule>().is_err());
        assert!("#.#.# => 1".parse::<Rule>().is_err());
        assert!("###123 => .".parse::<Rule>().is_err());

        // weird, but "correct" cases
        let data: Result<Rule, _> = "####### => #".parse();
        assert!(data.is_ok());

        let rule = data.unwrap();
        assert_eq!(rule.outcome, PlantState::Alive);
        assert_eq!(
            rule.pattern,
            [
                PlantState::Alive,
                PlantState::Alive,
                PlantState::Alive,
                PlantState::Alive,
                PlantState::Alive,
            ]
        );

        let data: Result<Rule, _> = "...#. => ####".parse();
        assert!(data.is_ok());

        let rule = data.unwrap();
        assert_eq!(rule.outcome, PlantState::Alive);
        assert_eq!(
            rule.pattern,
            [
                PlantState::Dead,
                PlantState::Dead,
                PlantState::Dead,
                PlantState::Alive,
                PlantState::Dead,
            ]
        );

        // weird, but "correct" cases
        let data: Result<Rule, _> = "#####213 => #".parse();
        assert!(data.is_ok());

        let rule = data.unwrap();
        assert_eq!(rule.outcome, PlantState::Alive);
        assert_eq!(
            rule.pattern,
            [
                PlantState::Alive,
                PlantState::Alive,
                PlantState::Alive,
                PlantState::Alive,
                PlantState::Alive,
            ]
        );
    }

    #[test]
    fn test_rule_apply() {
        macro_rules! state_parse {
            ($data: expr) => {
                $data
                    .chars()
                    .map(PlantState::from_char)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap()
            };
        }

        let rule = "...#. => .".parse::<Rule>().unwrap();

        let state = state_parse!("..#..");
        assert_eq!(rule.apply(&state), None);

        let state = state_parse!(".....");
        assert_eq!(rule.apply(&state), None);

        let state = state_parse!("#####");
        assert_eq!(rule.apply(&state), None);

        let state = state_parse!("...#.");
        assert_eq!(rule.apply(&state), Some(PlantState::Dead));

        let rule = "#..#. => #".parse::<Rule>().unwrap();

        let state = state_parse!("..#..");
        assert_eq!(rule.apply(&state), None);

        let state = state_parse!(".....");
        assert_eq!(rule.apply(&state), None);

        let state = state_parse!("#####");
        assert_eq!(rule.apply(&state), None);

        let state = state_parse!("...#.");
        assert_eq!(rule.apply(&state), None);

        let state = state_parse!("#..#.");
        assert_eq!(rule.apply(&state), Some(PlantState::Alive));
    }
}
