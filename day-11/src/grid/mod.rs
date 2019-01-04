use std::ops::{Index, Range};

use rayon::prelude::*;

mod cell;
use self::cell::Cell;

#[derive(Debug)]
pub(crate) struct Grid {
    size: usize,
    cells: Vec<i64>,
}

fn iter_product<T1: Copy, T2: Copy>(
    iter: impl Iterator<Item = T1> + Clone,
    other_iter: impl Iterator<Item = T2>,
) -> impl Iterator<Item = (T1, T2)> {
    other_iter
        .flat_map(move |i| iter.clone().map(move |j| (j, i)))
}

pub(crate) type Coordinates = (usize, usize);

pub(crate) type QuadrantPower = (Coordinates, i64);

impl Grid {
    pub fn new(size: usize, serial: i64) -> Self {
        // FIXME: currently storing extra (2 * size - 1) cells for computations simplicity
        let size = size + 1;
        let mut cells = Vec::with_capacity(size * size);

        for y in 0..size {
            for x in 0..size {
                cells.push(Cell::new(x, y, serial).power());
            }
        }

        Self {
            size,
            cells,
        }
    }

    pub fn find_global_maximum(&self) -> Option<(usize, QuadrantPower)> {
        // FIXME: should be something more intelligent, than bruteforce
        // but as for now it takes ~9 seconds on i7-8750H, so be it
        (1..(self.size - 1))
            .map(|quad_side| (quad_side, self.find_maximum(quad_side).unwrap()))
            .max_by_key(|(_side, (_coords, power))| *power)

    }

    pub fn find_maximum(&self, quadrant_side: usize) -> Option<QuadrantPower> {
        iter_product(
            1..(self.size - quadrant_side),
            1..(self.size - quadrant_side),
        )
        .collect::<Vec<_>>()
        .par_iter()
        .map(|&(x, y)| ((x, y), self.quadrant_sum(x, y, quadrant_side)))
        .max_by_key(|(_coords, power)| *power)
    }

    fn quadrant_sum(&self, x: usize, y: usize, quadrant_side: usize) -> i64 {
        (y..(y + quadrant_side))
            .map(|y| {
                self[(x..(x + quadrant_side), y)]
                    .iter()
                    .sum::<i64>()
            })
            .sum()
    }
}

impl Index<Coordinates> for Grid {
    type Output = i64;

    fn index(&self, index: Coordinates) -> &Self::Output {
        let (x, y) = index;

        debug_assert!(x < self.size);
        debug_assert!(y < self.size);

        &self.cells[y * self.size + x]
    }
}

impl Index<(Range<usize>, usize)> for Grid {
    type Output = [i64];

    fn index(&self, index: (Range<usize>, usize)) -> &Self::Output {
        let (x, y) = index;

        debug_assert!(x.start < self.size);
        debug_assert!(x.end < self.size);
        debug_assert!(y < self.size);

        let start = y * self.size + x.start;
        let end = y * self.size + x.end;

        &self.cells[start..end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let grid = Grid::new(4, 1);

        let range_indexed = grid[(0..3, 3)].iter().collect::<Vec<_>>();

        assert_eq!(
            range_indexed,
            [&grid[(0, 3)], &grid[(1, 3)], &grid[(2, 3)],]
        );
    }

    #[test]
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
    fn test_grid_max_examples() {
        let grid = Grid::new(300, 18);
        assert_eq!(grid.find_maximum(3).unwrap(), ((33, 45), 29));

        let grid = Grid::new(300, 42);
        assert_eq!(grid.find_maximum(3).unwrap(), ((21, 61), 30));
    }
}

