use std::cmp::Ordering;

pub type Scale = u32;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub struct Point {
    pub x: Scale,
    pub y: Scale,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct Line(pub Point, pub Point);

impl Line {
    /// Get all points along this line
    pub fn points_in_line(&self) -> impl Iterator<Item = Point> {
        PointsInLine {
            started: false,
            curr: self.0,
            end: self.1,
        }
    }
    pub fn is_straight(&self) -> bool {
        self.0.x == self.1.x || self.0.y == self.1.y
    }
}

/// An iterator that yields all points along the line that created it.
struct PointsInLine {
    curr: Point,
    end: Point,
    started: bool,
}

impl Iterator for PointsInLine {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            // First point
            self.started = true;
            return Some(self.curr);
        } else if self.curr == self.end {
            // Last point
            return None;
        }

        // move `curr` one step along the line, whichever direction
        // it needs to go.
        let x = &mut self.curr.x;
        match (*x).cmp(&self.end.x) {
            Ordering::Less => *x += 1,
            Ordering::Greater => *x -= 1,
            _ => {}
        }
        let y = &mut self.curr.y;
        match (*y).cmp(&self.end.y) {
            Ordering::Less => *y += 1,
            Ordering::Greater => *y -= 1,
            _ => {}
        };
        Some(self.curr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points_in_line() {
        let l = Line(Point { x: 0, y: 2 }, Point { x: 0, y: 4 });
        let expected = vec![
            Point { x: 0, y: 2 },
            Point { x: 0, y: 3 },
            Point { x: 0, y: 4 },
        ];
        let actual: Vec<_> = l.points_in_line().into_iter().collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_points_in_one_point_line() {
        let l = Line(Point { x: 0, y: 2 }, Point { x: 0, y: 2 });
        let expected = vec![Point { x: 0, y: 2 }];
        let actual: Vec<_> = l.points_in_line().into_iter().collect();
        assert_eq!(expected, actual);
    }
}
