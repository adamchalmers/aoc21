use std::{
    cmp::{min, Reverse},
    collections::HashSet,
    str::FromStr,
};

fn main() {
    let map_q1 = Grid::parse(include_str!("data/input.txt"));
    println!("Q1: {}", map_q1.lowest_risk_path());
    let map_q2 = map_q1.repeat_map(5);
    println!("Q2: {}", map_q2.lowest_risk_path());
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct Grid {
    x: usize,
    y: usize,
    cells: Vec<Vec<usize>>,
}

impl Grid {
    /// Iterate over all points in the grid.
    fn points(&self) -> impl Iterator<Item = Point> {
        let height = self.y;
        let width = self.x;
        (0..height).flat_map(move |y| (0..width).map(move |x| Point { x, y }))
    }

    /// Find all neighbouring points (not including diagonals).
    fn neighbours(&self, p: Point) -> impl Iterator<Item = Point> {
        let width = self.x as isize;
        let height = self.y as isize;
        let x = p.x as isize;
        let y = p.y as isize;
        [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .into_iter()
            .filter(move |(x, y)| x >= &0 && x < &width && y >= &0 && y < &height)
            .map(|(x, y)| Point {
                x: x as usize,
                y: y as usize,
            })
    }

    fn get(&self, p: Point) -> usize {
        self.cells[p.y][p.x]
    }

    /// Parse the text files from Advent of Code.
    fn parse(s: &str) -> Self {
        let cells: Vec<Vec<_>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| {
                        usize::from_str(&String::from(char)).expect("could not parse a char")
                    })
                    .collect()
            })
            .collect();

        let y = cells.len();
        let x = cells[0].len();
        Self { cells, x, y }
    }

    /// Calculate the lowest total risk of any path from the top left to the bottom right.
    /// Uses Djikstra's algorithm.
    fn lowest_risk_path(&self) -> usize {
        let start = Point { x: 0, y: 0 };
        let destination = Point {
            x: self.x - 1,
            y: self.y - 1,
        };

        // Initialize the search algorithm:
        // Mark all nodes unvisited. Create a set of all the unvisited nodes called the unvisited set.
        let mut tentative_distances = priority_queue::PriorityQueue::new();
        let mut visited: HashSet<Point> = Default::default();
        for p in self.points() {
            // Assign to every node a tentative distance value:
            // set it to zero for our initial node
            // and to infinity for all other nodes.
            // The tentative distance of a node v is the length of the shortest path discovered so
            // far between the node v and the starting node.
            tentative_distances.push(
                p,
                if p == start {
                    // The priority queue crate uses a maxheap, not a minheap, but we want to find
                    // the smallest distances. So instead of storing priority as usize, store it as
                    // Reverse<usize>, which reverses the comparisons.
                    Reverse(0)
                } else {
                    // Since initially no path is known to any other vertex than the source itself
                    // (which is a path of length zero), all other tentative distances are initially
                    // set to infinity.
                    Reverse(usize::MAX)
                },
            );
        }

        // Run the search algorithm:
        // Set the initial node as current.
        let mut curr = start;
        // Each iteration of this loop will mark current as visited and find its minimum distance.
        loop {
            // For the current node, consider all of its unvisited neighbors...
            for neighbour in self
                .neighbours(curr)
                .filter(|neighbour| !visited.contains(neighbour))
            {
                // ...and calculate their tentative distances through the current node.
                let dist_to_curr = tentative_distances
                    .get_priority(&curr)
                    // Use .0 to get the inner value (i.e. the distance) from inside the Reverse
                    // newtype wrapper.
                    .map(|d| d.0)
                    .unwrap();
                let dist_through_curr = dist_to_curr + self.get(neighbour);

                // Compare the newly calculated tentative distance to the current assigned value and
                // assign the smaller one.
                let old_dist = tentative_distances.get_priority(&neighbour).unwrap().0;
                let new_dist = min(old_dist, dist_through_curr);
                tentative_distances.change_priority(&neighbour, Reverse(new_dist));
            }
            // When we are done considering all of the unvisited neighbors of the current node,
            // mark the current node as visited and remove it from the unvisited set.
            // A visited node will never be checked again.
            visited.insert(curr);
            let (_, distance_to_curr) = tentative_distances.remove(&curr).unwrap();

            // If the destination node has been marked visited then stop.
            if curr == destination {
                return distance_to_curr.0;
            }

            // Otherwise, select the unvisited node that is marked with the smallest tentative
            // distance, set it as the new current node, and continue.
            curr = *tentative_distances.peek().unwrap().0;
        }
    }

    /// Tile the current map `n` times, with different risk in every tile.
    fn repeat_map(&self, n: usize) -> Self {
        let mut cells = vec![vec![0; self.x * n]; self.y * n];
        for cell_y in 0..self.y {
            for cell_x in 0..self.x {
                for region_y in 0..n {
                    for region_x in 0..n {
                        let original_risk = self.get(Point {
                            y: cell_y,
                            x: cell_x,
                        });
                        let x = cell_x + self.x * region_x;
                        let y = cell_y + self.y * region_y;
                        cells[y][x] = adjusted_risk(original_risk, region_x, region_y);
                    }
                }
            }
        }
        Self {
            x: self.x * n,
            y: self.y * n,
            cells,
        }
    }
}

/// Each time the tile repeats to the right or downward, all of its risk levels are 1 higher than
/// the tile immediately up or left of it. However, risk levels above 9 wrap back around to 1.
fn adjusted_risk(original: usize, x: usize, y: usize) -> usize {
    let a = original + x + y;
    let b = a % 10;
    if b < a {
        b + 1
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_iterator() {
        let g = Grid::parse(include_str!("data/example.txt"));
        let points_in_order: Vec<_> = g.points().collect();
        assert_eq!(points_in_order[0], Point { x: 0, y: 0 });
        assert_eq!(points_in_order[1], Point { x: 1, y: 0 });
        assert_eq!(
            points_in_order[points_in_order.len() - 1],
            Point { x: 9, y: 9 }
        );
    }

    #[test]
    fn test_q1() {
        let g = Grid::parse(include_str!("data/example.txt"));
        assert_eq!(g.lowest_risk_path(), 40)
    }

    #[test]
    fn test_repeat_map_trivial() {
        let expected = Grid::parse(include_str!("data/example.txt"));
        let actual = expected.repeat_map(1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_repeat_map() {
        let expected = Grid::parse(include_str!("data/example_q2.txt"));
        let small = Grid::parse(include_str!("data/example.txt"));
        let actual = small.repeat_map(5);
        assert_eq!(actual, expected);
        assert_eq!(actual.points().count(), 2500);
    }

    #[test]
    fn test_q2() {
        assert_eq!(
            Grid::parse(include_str!("data/example.txt"))
                .repeat_map(5)
                .lowest_risk_path(),
            315
        );
    }

    #[test]
    fn test_adjust_risk() {
        let answers = [
            [8, 9, 1, 2, 3],
            [9, 1, 2, 3, 4],
            [1, 2, 3, 4, 5],
            [2, 3, 4, 5, 6],
            [3, 4, 5, 6, 7],
        ];
        for (i, row) in answers.iter().enumerate() {
            for (j, expected) in row.iter().enumerate() {
                let actual = adjusted_risk(8, i, j);
                assert_eq!(*expected, actual, "{}", Point { x: i, y: j });
            }
        }
    }
}
