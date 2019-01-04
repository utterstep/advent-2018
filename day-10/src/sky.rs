use std::fmt;

use crate::star::Star;

#[derive(Debug)]
pub(crate) struct Sky {
    stars: Vec<Star>,
}

const VISUALIZATION_LIMIT: i64 = 70;

impl Sky {
    pub fn new(stars: Vec<Star>) -> Self {
        Self { stars }
    }

    fn x_bounds(&self) -> (i64, i64) {
        // TODO: minmax via reduce? also merge with advancing logic
        let x_min = self.stars.iter().map(|star| star.position.x).min().unwrap();
        let x_max = self.stars.iter().map(|star| star.position.x).max().unwrap();

        (x_min, x_max)
    }

    fn y_bounds(&self) -> (i64, i64) {
        // TODO: minmax via reduce? also merge with advancing logic
        let y_min = self.stars.iter().map(|star| star.position.y).min().unwrap();
        let y_max = self.stars.iter().map(|star| star.position.y).max().unwrap();

        (y_min, y_max)
    }

    fn y_span(&self) -> i64 {
        let (y_min, y_max) = self.y_bounds();

        y_max - y_min
    }

    pub fn advance_to_message(&mut self, visualize: bool) -> i64 {
        // assume that message is visible in the y-span local minimum
        let mut steps_taken = 0;
        let mut previous_span = self.y_span();
        let mut current_span = previous_span;

        while current_span <= previous_span {
            steps_taken += 1;

            self.advance();

            if visualize && current_span < VISUALIZATION_LIMIT {
                println!("{}\n\n{}\n\n", "=".repeat(100), self);
            }

            previous_span = current_span;
            current_span = self.y_span();
        }

        self.rewind();

        steps_taken - 1
    }

    fn advance(&mut self) {
        for star in self.stars.iter_mut() {
            star.advance();
        }
    }

    fn rewind(&mut self) {
        for star in self.stars.iter_mut() {
            star.rewind();
        }
    }
}

impl fmt::Display for Sky {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x_min, x_max) = self.x_bounds();
        let (y_min, y_max) = self.y_bounds();
        let x_span = x_max - x_min + 1;
        let y_span = y_max - y_min + 1;

        let cols = vec![' '; x_span as usize];
        let mut rows = (0..y_span).map(|_| cols.clone()).collect::<Vec<_>>();

        for star in &self.stars {
            rows[(y_max - star.position.y) as usize][(x_max - star.position.x) as usize] = '•';
        }

        let lines = rows
            .iter()
            .rev()
            .map(|row| row.iter().rev().collect::<String>());
        let output = lines.collect::<Vec<_>>().join("\n");

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sky_new() {
        let stars = vec![
            "position=<0, -3> velocity=<1, 2>".parse().unwrap(),
            "position=<-1, 0> velocity=<1, 2>".parse().unwrap(),
            "position=<0, 2> velocity=<1, 2>".parse().unwrap(),
            "position=<5, 1> velocity=<1, 2>".parse().unwrap(),
        ];

        let sky = Sky::new(stars);

        assert_eq!(sky.x_bounds(), (-1, 5));
        assert_eq!(sky.y_bounds(), (-3, 2));
        assert_eq!(sky.y_span(), 5);
    }
}
