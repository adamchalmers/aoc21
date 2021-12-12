mod counter;

use cached::proc_macro::cached;
use counter::Counter;
use std::collections::HashSet;

fn main() {
    let g = Grid::parse(include_str!("input.txt"));
    println!("Q1: {}", g.total_low_point_risk());
    println!("Q2: {}", solve_q2(g));
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

#[derive(Hash, PartialEq, Eq, Clone)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<u16>>,
}

impl Grid {
    fn new(x: usize, y: usize) -> Self {
        Self {
            width: x,
            height: y,
            cells: vec![vec![0; x]; y],
        }
    }

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
    fn adjacent(&self, p: Point) -> HashSet<Point> {
        let mut out = HashSet::new();
        if p.x < self.width - 1 {
            out.insert(Point { x: p.x + 1, ..p });
        }
        if p.x > 0 {
            out.insert(Point { x: p.x - 1, ..p });
        }
        if p.y < self.height - 1 {
            out.insert(Point { y: p.y + 1, ..p });
        }
        if p.y > 0 {
            out.insert(Point { y: p.y - 1, ..p });
        }
        out
    }
    /// Iterate over all points in this grid.
    fn all_points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.height).flat_map(|y| (0..self.width).map(move |x| Point { x, y }))
    }
    fn get(&self, p: Point) -> u16 {
        self.cells[p.y][p.x]
    }
}

fn find_low_points(g: &Grid) -> Vec<Point> {
    g.all_points()
        .filter(|p| g.adjacent(*p).iter().all(|adj| g.get(*adj) > g.get(*p)))
        .collect()
}

impl Grid {
    fn total_low_point_risk(&self) -> u16 {
        find_low_points(self)
            .into_iter()
            .map(|lp| self.get(lp) + 1)
            .sum()
    }
}

fn downhill_from(g: &Grid, p: Point) -> Option<Point> {
    if g.get(p) == 9 {
        return None;
    }
    let curr_depth = g.get(p);
    let next = g
        .adjacent(p)
        .into_iter()
        .map(|adj| (adj, g.get(adj)))
        .filter(|(_adj, depth)| depth < &curr_depth)
        .min_by(|(_p1, val1), (_p2, val2)| val1.cmp(val2))
        .map(|(p, _)| p)?;
    Some(next)
}

#[cached]
fn basin_of(g: Grid, p: Point) -> Option<Point> {
    let next = match downhill_from(&g, p) {
        Some(next) => next,
        None => return Some(p),
    };
    let basin = basin_of(g, next)?;
    Some(basin)
}

fn all_basins_sizes(g: Grid) -> Vec<(Point, u16)> {
    let every_basin = g.all_points().filter_map(|p| basin_of(g.clone(), p));
    let mut c: Counter<Point> = Default::default();
    for basin in every_basin {
        c.add(basin);
    }
    c.0.into_iter().collect()
}

fn solve_q2(g: Grid) -> u32 {
    let mut basins: Vec<_> = all_basins_sizes(g)
        .into_iter()
        .map(|(_basin, num)| num)
        .collect();
    basins.sort_unstable();
    basins.reverse();
    basins.into_iter().take(3).map(|size| size as u32).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_example() {
        let g: Grid = Grid::parse(include_str!("example.txt"));
        assert_eq!(g.total_low_point_risk(), 15)
    }

    #[test]
    fn q2_downhills() {
        let g: Grid = Grid::parse(include_str!("example.txt"));
        let start = Point { x: 0, y: 1 };
        let next = Point { x: 0, y: 0 };
        let end = Point { x: 1, y: 0 };
        assert_eq!(downhill_from(&g, start), Some(next));
        assert_eq!(downhill_from(&g, next), Some(end));
        assert_eq!(downhill_from(&g, end), None);
    }

    #[test]
    fn q2_example() {
        let g: Grid = Grid::parse(include_str!("example.txt"));
        let answer = solve_q2(g);
        assert_eq!(answer, 1134);
    }
}
