#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, x: i32, y: i32) -> i32 {
        (self.x - x).abs() + (self.y - y).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_new() {
        assert_eq!(Point::new(5, 0), Point { x: 5, y: 0 });
    }

    #[test]
    fn test_distance_to() {
        let p = Point::new(3, 3);

        assert_eq!(p.distance_to(0, 0), 6);
        assert_eq!(p.distance_to(3, 3), 0);
        assert_eq!(p.distance_to(6, 6), 6);
        assert_eq!(p.distance_to(2, 1), 3);
        assert_eq!(p.distance_to(-100, -100), 206);
        assert_eq!(p.distance_to(20, 0), 20);
    }
}
