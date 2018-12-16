#[derive(Debug)]
pub(super) struct Boundaries {
    top: i32,
    right: i32,
    bottom: i32,
    left: i32,
}

impl Boundaries {
    pub(super) fn new() -> Self {
        Self {
            top: i32::min_value(),
            right: i32::min_value(),
            bottom: i32::max_value(),
            left: i32::max_value(),
        }
    }

    pub(super) fn include_point(&mut self, x: i32, y: i32) {
        if x < self.left {
            self.left = x;
        }
        if x > self.right {
            self.right = x;
        }

        if y < self.bottom {
            self.bottom = y;
        }
        if y > self.top {
            self.top = y;
        }
    }

    pub(super) fn is_inner_point(&self, x: i32, y: i32) -> bool {
        x >= self.left && x <= self.right &&
        y >= self.bottom && x <= self.top
    }

    pub(super) fn is_edge_point(&self, x: i32, y: i32) -> bool {
        self.is_inner_point(x, y) &&
        (
            self.left == x || self.right == x ||
            self.top == y || self.bottom == y
        )
    }
}

impl<'a> IntoIterator for &'a Boundaries {
    type Item = (i32, i32);
    type IntoIter = BoundariesIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BoundariesIterator {
            boundaries: &self,
            current_x: self.left - 1, // to start from first point
            current_y: self.bottom,
        }
    }
}

pub struct BoundariesIterator<'a> {
    boundaries: &'a Boundaries,
    current_x: i32,
    current_y: i32,
}

impl Iterator for BoundariesIterator<'_> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.current_x += 1;

        if self.current_x <= self.boundaries.right {
            return Some((self.current_x, self.current_y));
        }

        self.current_x = self.boundaries.left;
        self.current_y += 1;

        if self.current_y <= self.boundaries.top {
            Some((self.current_x, self.current_y))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boundaries() {
        let mut b = Boundaries::new();
        assert!(!b.is_inner_point(0, 0));
        assert!(!b.is_edge_point(0, 0));

        b.include_point(0, 0);
        assert!(b.is_inner_point(0, 0));
        assert!(b.is_edge_point(0, 0));

        b.include_point(5, 5);
        assert!(b.is_edge_point(5, 5));
        assert!(!b.is_edge_point(3, 3));
        assert!(b.is_inner_point(3, 3));

        let mut b = Boundaries::new();
        assert!(!b.is_edge_point(0, 0));

        b.include_point(3, 3);
        assert!(!b.is_edge_point(0, 0));
        assert!(b.is_edge_point(3, 3));

        b.include_point(10, 10);
        assert!(!b.is_edge_point(0, 0));
        assert!(b.is_edge_point(10, 4));
        assert!(!b.is_edge_point(10, 0));
        assert!(b.is_edge_point(5, 10));
        assert!(b.is_inner_point(5, 5));
    }

    #[test]
    fn test_boundaries_iteration() {
        let mut b = Boundaries::new();
        b.include_point(13, 22);
        b.include_point(3, 11);
        b.include_point(11, 15);

        let mut should_iterate_over = Vec::new();

        for y in b.bottom..=b.top {
            for x in b.left..=b.right {
                should_iterate_over.push((x, y));
            }
        }

        assert_eq!(
            b.into_iter().collect::<Vec<_>>(),
            should_iterate_over
        );
    }
}
