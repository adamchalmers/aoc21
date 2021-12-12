use std::collections::HashSet;

fn main() {
    let g: Grid<u8, 100, 100> = Grid::parse(include_str!("input.txt"));
    println!("Q1: {}", g.total_low_point_risk());
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Default, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

struct Grid<T, const X: usize, const Y: usize>([[T; X]; Y]);

impl<T, const X: usize, const Y: usize> Default for Grid<T, X, Y>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self([[T::default(); X]; Y])
    }
}

impl<const X: usize, const Y: usize> Grid<u8, X, Y> {
    fn parse(s: &str) -> Self {
        let mut g = Self::default();
        for (i, line) in s.lines().enumerate() {
            for (j, char) in line.chars().enumerate() {
                g.0[i][j] = String::from(char).parse().unwrap();
            }
        }
        g
    }
}

impl<T, const X: usize, const Y: usize> Grid<T, X, Y> {
    fn adjacent(&self, p: Point) -> HashSet<Point> {
        let mut out = HashSet::new();
        if p.x < X - 1 {
            out.insert(Point { x: p.x + 1, ..p });
        }
        if p.x > 0 {
            out.insert(Point { x: p.x - 1, ..p });
        }
        if p.y < Y - 1 {
            out.insert(Point { y: p.y + 1, ..p });
        }
        if p.y > 0 {
            out.insert(Point { y: p.y - 1, ..p });
        }
        out
    }
    /// Iterate over all points in this grid.
    fn all_points(&self) -> impl Iterator<Item = Point> {
        (0..Y).flat_map(|y| (0..X).map(move |x| Point { x, y }))
    }
    fn get(&self, p: Point) -> &T {
        &self.0[p.y][p.x]
    }
}

impl<T, const X: usize, const Y: usize> Grid<T, X, Y>
where
    T: Ord,
{
    fn find_low_points(&self) -> impl Iterator<Item = Point> + '_ {
        self.all_points().filter(|p| {
            self.adjacent(*p)
                .iter()
                // .inspect(|adj| if p.y == 4 && p.x == )
                .all(|adj| self.get(*adj) > self.get(*p))
        })
    }
}

impl<const X: usize, const Y: usize> Grid<u8, X, Y> {
    fn total_low_point_risk(&self) -> u16 {
        self.find_low_points()
            .map(|lp| *self.get(lp) as u16 + 1)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_points() {
        let g: Grid<(), 2, 2> = Default::default();
        assert_eq!(16, g.all_points().count())
    }

    #[test]
    fn test_adj() {
        let g: Grid<(), 3, 3> = Default::default();
        assert_eq!(
            g.adjacent(Point::default()),
            HashSet::from([Point { x: 0, y: 1 }, Point { x: 1, y: 0 }])
        );
        assert_eq!(
            g.adjacent(Point { x: 2, y: 4 }),
            HashSet::from([
                Point { x: 1, y: 4 },
                Point { x: 3, y: 4 },
                Point { x: 2, y: 3 }
            ])
        );
    }

    #[test]
    fn q1_example() {
        let g: Grid<u8, 10, 5> = Grid::parse(include_str!("example.txt"));
        assert_eq!(g.total_low_point_risk(), 15)
    }
}
