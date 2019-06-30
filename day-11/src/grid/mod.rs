mod cell;
use self::cell::Cell;

#[derive(Debug)]
pub(crate) struct Grid {
    size: usize,
    cells: Vec<i64>,
    sums: Vec<i64>,
}

fn iter_product<T1: Copy, T2: Copy>(
    iter: impl Iterator<Item = T1> + Clone,
    other_iter: impl Iterator<Item = T2>,
) -> impl Iterator<Item = (T1, T2)> {
    other_iter.flat_map(move |i| iter.clone().map(move |j| (j, i)))
}

pub(crate) type Coordinates = (usize, usize);

pub(crate) type QuadrantPower = (Coordinates, i64);

macro_rules! idx {
    ($x: expr, $y: expr, $size: expr) => {
        $y * $size + $x;
    };
}

fn make_sums_table(size: usize, cells: &[i64]) -> Vec<i64> {
    debug_assert_eq!((cells.len() as f64).sqrt() as usize, size);

    let mut sums = vec![0; cells.len()];
    sums[0] = cells[0];

    for x in 1..size {
        sums[x] = sums[x - 1] + cells[x];
    }

    for y in 1..size {
        sums[idx!(0, y, size)] = sums[idx!(0, y - 1, size)] + cells[idx!(0, y, size)];
    }

    for x in 1..size {
        for y in 1..size {
            sums[idx!(x, y, size)] =
                sums[idx!(x - 1, y, size)] + sums[idx!(x, y - 1, size)] + cells[idx!(x, y, size)]
                    - sums[idx!(x - 1, y - 1, size)];
        }
    }

    sums
}

impl Grid {
    pub fn new(size: usize, serial: i64) -> Self {
        let mut cells = Vec::with_capacity(size * size);

        for y in 0..size {
            for x in 0..size {
                cells.push(Cell::new(x + 1, y + 1, serial).power());
            }
        }

        let sums = make_sums_table(size, &cells);

        Self { size, cells, sums }
    }

    pub fn find_global_maximum(&self) -> Option<(usize, QuadrantPower)> {
        (1..self.size)
            .map(|quad_side| (quad_side, self.find_maximum(quad_side).unwrap()))
            .max_by_key(|(_side, (_coords, power))| *power)
    }

    pub fn find_maximum(&self, quadrant_side: usize) -> Option<QuadrantPower> {
        iter_product(
            0..=(self.size - quadrant_side),
            0..=(self.size - quadrant_side),
        )
        // Re: previously next code snipped had +2 bias
        // After a good night sleep I understood this weirdness:
        // * first +1 is obviously from Cell creation (see Grid::new)
        // * second is from Grid::quadrant_sum(x, y, side) being really
        //   a sum from x + 1, y + 1 to x + side, y + side
        .map(|(x, y)| ((x + 1, y + 1), self.quadrant_sum(x, y, quadrant_side)))
        .max_by_key(|(_coords, power)| *power)
    }

    fn quadrant_sum(&self, x: usize, y: usize, quadrant_side: usize) -> i64 {
        // FIXME: this `if` has ~1.8x performance penalty (33ms vs 18ms)
        // and is used here only for correctness sake
        if x & y == 0 {
            return self.sums[idx!(x, y, self.size)];
        }

        let x = x - 1;
        let y = y - 1;

        self.sums[idx!(x + quadrant_side, y + quadrant_side, self.size)]
            - self.sums[idx!(x, y + quadrant_side, self.size)]
            - self.sums[idx!(x + quadrant_side, y, self.size)]
            + self.sums[idx!(x, y, self.size)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_sums_table() {
        // 0 0 0
        // 0 0 0
        // 0 0 0
        // =>
        // 0 0 0
        // 0 0 0
        // 0 0 0
        let zeros = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
        let zeros_sum = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];

        assert_eq!(make_sums_table(3, &zeros), zeros_sum,);

        // 1 1 1
        // 1 1 1
        // 1 1 1
        // =>
        // 1 2 3
        // 2 4 6
        // 3 6 9
        let ones = vec![1, 1, 1, 1, 1, 1, 1, 1, 1];
        let ones_sum = vec![1, 2, 3, 2, 4, 6, 3, 6, 9];

        assert_eq!(make_sums_table(3, &ones), ones_sum,);

        // 1 2 3
        // 4 5 6
        // 7 8 9
        // =>
        //  1  3  6
        //  5 12 21
        // 12 27 45
        let increasing = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let increasing_sum = vec![1, 3, 6, 5, 12, 21, 12, 27, 45];

        assert_eq!(make_sums_table(3, &increasing), increasing_sum,);
    }

    #[test]
    fn test_ranges_product() {
        assert_eq!(
            iter_product(0..4, 0..2).collect::<Vec<_>>(),
            vec![
                (0, 0),
                (1, 0),
                (2, 0),
                (3, 0),
                (0, 1),
                (1, 1),
                (2, 1),
                (3, 1),
            ]
        );
    }

    #[test]
    fn test_grid() {
        let grid = Grid::new(10, 8);

        assert_eq!(grid.cells[idx!(2, 4, 10)], 4);
    }

    #[test]
    fn test_grid_max_examples() {
        let grid = Grid::new(300, 42);
        assert_eq!(grid.find_maximum(3).unwrap(), ((21, 61), 30));
        assert_eq!(grid.find_global_maximum().unwrap(), (12, ((232, 251), 119)));

        let grid = Grid::new(300, 18);
        assert_eq!(grid.find_maximum(3).unwrap(), ((33, 45), 29));
        assert_eq!(grid.find_global_maximum().unwrap(), (16, ((90, 269), 113)));
    }
}
