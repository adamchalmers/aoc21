mod counter;

use cached::proc_macro::cached;
use counter::Counter;

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

/// 2D grid type
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

    /// Parse from AoC problem input textfile.
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

    /// Get all cells adjacent to the given point.
    /// Usually there will be 4, but if the point is an edge/corner there will only be
    /// 3 or 2.
    fn adjacent(&self, p: Point) -> impl Iterator<Item = Point> {
        let mut out = Vec::new();
        if p.x < self.width - 1 {
            out.push(Point { x: p.x + 1, ..p });
        }
        if p.x > 0 {
            out.push(Point { x: p.x - 1, ..p });
        }
        if p.y < self.height - 1 {
            out.push(Point { y: p.y + 1, ..p });
        }
        if p.y > 0 {
            out.push(Point { y: p.y - 1, ..p });
        }
        out.into_iter()
    }

    /// Iterate over all points in this grid.
    fn all_points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.height).flat_map(|y| (0..self.width).map(move |x| Point { x, y }))
    }

    fn get(&self, p: Point) -> u16 {
        self.cells[p.y][p.x]
    }

    fn total_low_point_risk(&self) -> u16 {
        find_low_points(self)
            .into_iter()
            .map(|lp| self.get(lp) + 1)
            .sum()
    }
}

/// Find all points which have no downhill neighbours.
fn find_low_points(g: &Grid) -> Vec<Point> {
    g.all_points()
        .filter(|p| g.adjacent(*p).all(|adj| g.get(adj) > g.get(*p)))
        .collect()
}

/// Find the neighbour most steeply downhill from this point,
/// if one exists.
fn downhill_from(g: &Grid, p: Point) -> Option<Point> {
    if g.get(p) == 9 {
        return None;
    }
    let curr_depth = g.get(p);
    g.adjacent(p)
        .into_iter()
        .map(|adj| (adj, g.get(adj)))
        .filter(|(_adj, depth)| depth < &curr_depth)
        .min_by(|(_p1, val1), (_p2, val2)| val1.cmp(val2))
        .map(|(p, _)| p)
}

/// Find the basin this point is part of.
#[cached]
fn basin_of(g: Grid, point: Point) -> Option<Point> {
    let downhill = match downhill_from(&g, point) {
        Some(p) => p,
        None => return Some(point),
    };
    basin_of(g, downhill)
}

/// Get the size of every basin in this grid.
fn all_basins_sizes(g: Grid) -> Vec<u16> {
    let every_basin = g.all_points().filter_map(|p| basin_of(g.clone(), p));
    let mut c: Counter<Point> = Default::default();
    for basin in every_basin {
        c.add(basin);
    }
    c.0.into_iter().map(|(_basin, size)| size).collect()
}

fn solve_q2(g: Grid) -> u32 {
    let mut basins = all_basins_sizes(g);
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
