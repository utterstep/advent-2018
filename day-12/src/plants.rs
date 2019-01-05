#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(crate) enum PlantState {
    Dead,
    Alive,
}

impl PlantState {
    pub fn from_char(c: char) -> Result<Self, char> {
        match c {
            '#' => Ok(PlantState::Alive),
            '.' => Ok(PlantState::Dead),
            other_char => Err(other_char),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plant_state_from_char() {
        let good = PlantState::from_char('#');

        assert!(good.is_ok());
        assert_eq!(good.unwrap(), PlantState::Alive);

        let good = PlantState::from_char('.');

        assert!(good.is_ok());
        assert_eq!(good.unwrap(), PlantState::Dead);

        assert!(PlantState::from_char('1').is_err());
        assert!(PlantState::from_char('x').is_err());
        assert!(PlantState::from_char('s').is_err());
    }
}
