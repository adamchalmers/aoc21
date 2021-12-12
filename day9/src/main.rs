use cached::proc_macro::cached;
use std::collections::HashSet;

fn main() {
    let g: Grid<u8> = Grid::parse(include_str!("input.txt"));
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

struct Grid<T> {
    X: usize,
    Y: usize,
    cells: Vec<Vec<T>>,
}
impl<T> Grid<T>
where
    T: Default + Clone,
{
    fn new(x: usize, y: usize) -> Self {
        Self {
            X: x,
            Y: y,
            cells: vec![vec![T::default(); x]; y],
        }
    }
}
impl Grid<u8> {
    fn parse(s: &str) -> Self {
        let y = s.lines().count();
        let x = s.lines().next().unwrap().len();
        let mut g = Self::new(x, y);
        for (i, line) in s.lines().enumerate() {
            for (j, char) in line.chars().enumerate() {
                g.cells[i][j] = String::from(char).parse().unwrap();
            }
        }
        g
    }
}

impl<T> Grid<T> {
    fn adjacent(&self, p: Point) -> HashSet<Point> {
        let mut out = HashSet::new();
        if p.x < self.X - 1 {
            out.insert(Point { x: p.x + 1, ..p });
        }
        if p.x > 0 {
            out.insert(Point { x: p.x - 1, ..p });
        }
        if p.y < self.Y - 1 {
            out.insert(Point { y: p.y + 1, ..p });
        }
        if p.y > 0 {
            out.insert(Point { y: p.y - 1, ..p });
        }
        out
    }
    /// Iterate over all points in this grid.
    fn all_points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.Y).flat_map(|y| (0..self.X).map(move |x| Point { x, y }))
    }
    fn get(&self, p: Point) -> &T {
        &self.cells[p.y][p.x]
    }
}

impl<T> Grid<T>
where
    T: Ord,
{
    fn find_low_points(&self) -> impl Iterator<Item = Point> + '_ {
        self.all_points().filter(|p| {
            self.adjacent(*p)
                .iter()
                .all(|adj| self.get(*adj) > self.get(*p))
        })
    }
}

impl Grid<u8> {
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
    fn q1_example() {
        let g: Grid<u8> = Grid::parse(include_str!("example.txt"));
        assert_eq!(g.total_low_point_risk(), 15)
    }
}
