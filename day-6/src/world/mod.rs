use std::collections::{HashMap, HashSet};

mod boundaries;

use self::boundaries::Boundaries;

use super::Point;

#[derive(Debug)]
pub struct World {
    points: HashMap<Point, i32>,
    infinite_areas_centers: HashSet<Point>,
    boundaries: Boundaries,
}

impl World {
    fn new() -> Self {
        Self {
            points: HashMap::new(),
            infinite_areas_centers: HashSet::new(),
            boundaries: Boundaries::new(),
        }
    }

    fn insert_points(&mut self, points: Vec<Point>) {
        for point in points {
            self.boundaries.include_point(point.x, point.y);

            self.points.insert(point, 0);
        }
    }

    pub fn from_points(points: Vec<Point>) -> Self {
        let mut world = Self::new();

        world.insert_points(points);

        world
    }

    fn compute_points_area(&mut self) {
        for (x, y) in &self.boundaries {
            let mut min_distance = i32::max_value();
            let mut closest = None;

            for (point, area) in self.points.iter_mut() {
                let distance = point.distance_to(x, y);

                if distance == min_distance {
                    closest = None;
                } else if distance < min_distance {
                    closest.replace((point, area));

                    min_distance = distance;
                }
            }

            if let Some((point, area)) = closest {
                if self.boundaries.is_edge_point(x, y) {
                    self.infinite_areas_centers.insert((*point).clone());
                }

                *area += 1;
            }
        }
    }

    pub fn largest_finite_area(mut self) -> Option<i32> {
        self.compute_points_area();

        self.points
            .iter()
            .filter(|(p, area)| **area != 0 && !self.infinite_areas_centers.contains(p))
            .max_by_key(|(_, area)| *area)
            .map(|(_, area)| *area)
    }

    pub fn safe_area_size(&self, max_distance: i32) -> usize {
        self.boundaries
            .into_iter()
            .filter(|(x, y)| {
                self.points
                    .keys()
                    .map(|point| point.distance_to(*x, *y))
                    .sum::<i32>()
                    < max_distance
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_empty_inside() {
        // world with no inner points
        let points = vec![
            "0, 0".parse().unwrap(),
            "0, 1".parse().unwrap(),
            "1, 0".parse().unwrap(),
            "1, 1".parse().unwrap(),
        ];

        let world = World::from_points(points);

        println!("{:?}", world);

        assert_eq!(world.largest_finite_area(), None);
    }

    #[test]
    fn test_world_center_only() {
        // world with center only
        let points = vec![
            "0, 0".parse().unwrap(),
            "0, 4".parse().unwrap(),
            "4, 0".parse().unwrap(),
            "4, 4".parse().unwrap(),
            "2, 2".parse().unwrap(),
        ];

        let world = World::from_points(points);

        assert_eq!(world.largest_finite_area(), Some(5));
    }

    #[test]
    fn test_example_world() {
        // https://adventofcode.com/2018/day/6
        let points = vec![
            "1, 1".parse().unwrap(),
            "1, 6".parse().unwrap(),
            "8, 3".parse().unwrap(),
            "3, 4".parse().unwrap(),
            "5, 5".parse().unwrap(),
            "8, 9".parse().unwrap(),
        ];

        let world = World::from_points(points);

        assert_eq!(world.largest_finite_area(), Some(17));
    }

    #[test]
    fn test_example_world_safe() {
        // https://adventofcode.com/2018/day/6
        let points = vec![
            "1, 1".parse().unwrap(),
            "1, 6".parse().unwrap(),
            "8, 3".parse().unwrap(),
            "3, 4".parse().unwrap(),
            "5, 5".parse().unwrap(),
            "8, 9".parse().unwrap(),
        ];

        let world = World::from_points(points);

        assert_eq!(world.safe_area_size(32), 16);
    }
}
