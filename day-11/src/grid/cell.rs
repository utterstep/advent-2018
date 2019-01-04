const POWER_DELTA: i64 = 5;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Cell {
    pub(super) x: usize,
    pub(super) y: usize,
    power: i64,
}

impl Cell {
    pub(super) fn new(x: usize, y: usize, serial: i64) -> Self {
        let rack_id = (x + 10) as i64;

        let power = (rack_id * y as i64 + serial) * rack_id;
        let power = (power % 1000) / 100 - POWER_DELTA;

        Self { x, y, power }
    }

    pub(super) fn power(&self) -> i64 {
        self.power
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell() {
        macro_rules! test_cell_example {
            ($x: expr, $y: expr, $serial: expr, $expected_power: expr) => {
                let cell = Cell::new($x, $y, $serial);

                assert_eq!(cell.x, $x);
                assert_eq!(cell.y, $y);
                assert_eq!(cell.power, $expected_power);
                assert_eq!(cell.power(), $expected_power);
            };
        }

        let cell = Cell::new(3, 5, 8);

        assert_eq!(cell.x, 3);
        assert_eq!(cell.y, 5);
        assert_eq!(cell.power, 4);
        assert_eq!(cell.power(), 4);

        test_cell_example!(3, 5, 8, 4);

        // Fuel cell at  122,79, grid serial number 57: power level -5.
        // Fuel cell at 217,196, grid serial number 39: power level  0.
        // Fuel cell at 101,153, grid serial number 71: power level  4.
        test_cell_example!(122, 79, 57, -5);
        test_cell_example!(217, 196, 39, 0);
        test_cell_example!(101, 153, 71, 4);
    }
}
