use std::collections::HashSet;

use crate::claim::Claim;

const CONFLICT: i32 = -1;

pub struct Fabric {
    cells: Vec<Vec<i32>>,
    valid_claims: HashSet<i32>,
}

impl Fabric {
    pub fn new(capacity: usize) -> Self {
        Self {
            cells: vec![vec![0; capacity]; capacity],
            valid_claims: HashSet::new(),
        }
    }

    pub fn process_claim(&mut self, claim: &Claim) -> usize {
        let mut conflicts = 0;
        let mut is_valid = true;

        for x in claim.left..(claim.left + claim.width) {
            for y in claim.top..(claim.top + claim.height) {
                match self.cells[x][y] {
                    0 => self.cells[x][y] = claim.number,
                    -1 => {
                        is_valid = false;
                    }
                    previously_taken => {
                        self.cells[x][y] = CONFLICT;
                        self.valid_claims.remove(&previously_taken);

                        conflicts += 1;
                        is_valid = false;
                    }
                }
            }
        }

        if is_valid {
            self.valid_claims.insert(claim.number);
        }

        conflicts
    }

    pub fn count_conflicts(&self) -> usize {
        self.cells.iter().fold(0, |acc, row| {
            acc + row
                .iter()
                .fold(0, |acc, cell| acc + if cell == &CONFLICT { 1 } else { 0 })
        })
    }

    pub fn valid_claims(&self) -> impl Iterator<Item = &i32> {
        self.valid_claims.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_claims() -> Vec<Claim> {
        vec![
            Claim {
                number: 1,
                top: 1,
                left: 3,
                width: 4,
                height: 4,
            },
            Claim {
                number: 2,
                top: 3,
                left: 1,
                width: 4,
                height: 4,
            },
            Claim {
                number: 3,
                top: 5,
                left: 5,
                width: 2,
                height: 2,
            },
        ]
    }

    #[test]
    fn test_process_claim() {
        let mut fabric = Fabric::new(10);
        let claim = Claim {
            number: 1,
            top: 1,
            left: 3,
            width: 4,
            height: 4,
        };

        assert_eq!(fabric.process_claim(&claim), 0);
        assert_eq!(fabric.process_claim(&claim), claim.width * claim.height);
    }

    #[test]
    fn test_example_conflicts() {
        let mut fabric = Fabric::new(10);
        let claims = get_example_claims();

        let mut conflicts = 0;

        for claim in claims {
            conflicts += fabric.process_claim(&claim);
        }

        assert_eq!(fabric.count_conflicts(), 4);
        assert_eq!(conflicts, 4);
    }

    #[test]
    fn test_valid_claims() {
        let mut fabric = Fabric::new(10);
        let claims = get_example_claims();

        for claim in claims {
            fabric.process_claim(&claim);
        }

        assert_eq!(fabric.valid_claims().collect::<Vec<_>>(), vec![&3]);
    }
}
