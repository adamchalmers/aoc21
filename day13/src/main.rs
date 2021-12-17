use std::collections::HashSet;

fn main() {
    let problem = Problem::parse(include_str!("data/input.txt"));
    let folded_points = problem.folds[0].apply(problem.holes.clone());
    println!("Q1: {}", folded_points.len());
    print(&problem.solve());
}

/// A point on the 2D plane.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

/// An axis of the 2D plane.
enum Axis {
    X,
    Y,
}

impl TryFrom<char> for Axis {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'x' => Ok(Self::X),
            'y' => Ok(Self::Y),
            other => Err(other),
        }
    }
}

/// The data given for this AoC problem.
struct Problem {
    holes: HashSet<Point>,
    folds: Vec<Fold>,
}

/// A way to fold the paper.
struct Fold {
    /// Along which axis should you fold?
    dir: Axis,
    /// At what position along the given axis should you fold?
    val: usize,
}

impl Fold {
    /// Does this fold keep the hole unchanged?
    /// False if folding would change this hole's position.
    fn unchanged(&self, p: &Point) -> bool {
        match self.dir {
            Axis::X => p.x < self.val,
            Axis::Y => p.y < self.val,
        }
    }

    /// Reflect the point around the fold line.
    fn reflect(&self, point: Point) -> Point {
        match self.dir {
            Axis::X => Point {
                x: 2 * self.val - point.x,
                ..point
            },
            Axis::Y => Point {
                y: 2 * self.val - point.y,
                ..point
            },
        }
    }

    /// Fold the set of holes along this fold.
    fn apply(&self, points: HashSet<Point>) -> HashSet<Point> {
        let (to_keep, to_change) = points
            .into_iter()
            .partition::<HashSet<_>, _>(|p| self.unchanged(p));

        // Keep the points above the fold,
        // Change the points that got folded.
        let mut new_points = to_keep;
        for point in to_change {
            new_points.insert(self.reflect(point));
        }
        new_points
    }
}

impl Problem {
    /// Parse from the Advent of Code text file.
    fn parse(s: &str) -> Self {
        let mut lines = s.lines();
        let mut holes = HashSet::new();
        loop {
            let line = lines.next().unwrap();
            if line.is_empty() {
                break;
            }
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            holes.insert(Point { x, y });
        }
        const FOLD_PREFIX: &str = "fold along ";
        let folds = lines
            .map(|l| {
                let line = l.chars().collect::<Vec<_>>();
                let dir = line[FOLD_PREFIX.len()].try_into().unwrap();
                let val = line[FOLD_PREFIX.len() + 2..]
                    .to_vec()
                    .into_iter()
                    .collect::<String>()
                    .parse()
                    .unwrap();
                Fold { dir, val }
            })
            .collect();
        Self { holes, folds }
    }

    /// Apply all folds to the set of holes.
    fn solve(self) -> HashSet<Point> {
        self.folds
            .into_iter()
            .fold(self.holes, |holes, fold| fold.apply(holes))
    }
}

/// Pretty-print the holes, displaying them on a grid.
fn print(points: &HashSet<Point>) {
    let width = points.iter().map(|p| p.x).max().unwrap();
    let height = points.iter().map(|p| p.y).max().unwrap();
    for y in 0..=height {
        let mut line = Vec::new();
        for x in 0..=width {
            line.push(if points.contains(&Point { x, y }) {
                '#'
            } else {
                '.'
            });
        }
        println!("{}", line.into_iter().collect::<String>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let p = Problem::parse(include_str!("data/example.txt"));
        assert_eq!(p.holes.len(), 18);
    }

    #[test]
    fn test_q1() {
        let problem = Problem::parse(include_str!("data/example.txt"));
        let folded_points = problem.folds[0].apply(problem.holes);
        assert_eq!(17, folded_points.len());
    }

    #[test]
    fn test_q2() {
        let problem = Problem::parse(include_str!("data/example.txt"));
        print(&problem.solve());
    }
}
