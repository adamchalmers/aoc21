use std::collections::{HashMap, HashSet};

fn main() {
    let g = Graph::parse_input(include_str!("data/input.txt"));
    println!("Q1: {}", g.paths(SmallCavesTwice::Never));
    println!("Q2: {}", g.paths(SmallCavesTwice::OnlyOnce));
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum Cave {
    Start,
    End,
    Big(String),
    Small(String),
}

impl Cave {
    fn parse(s: &str) -> Self {
        match s {
            "start" => Self::Start,
            "end" => Self::End,
            cave if cave.to_uppercase() == cave => Self::Big(cave.to_owned()),
            cave => Self::Small(cave.to_owned()),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum SmallCavesTwice {
    /// For part 1
    Never,
    /// For part 2
    OnlyOnce,
}

struct Graph {
    edges: HashSet<(Cave, Cave)>,
}

impl Graph {
    fn parse_input(s: &str) -> Self {
        let edges = s
            .lines()
            .map(|line| {
                let mut caves = line.split('-').take(2).map(Cave::parse);
                (caves.next().unwrap(), caves.next().unwrap())
            })
            .collect();
        Self { edges }
    }

    fn paths(&self, rule: SmallCavesTwice) -> usize {
        self.dfs(Path::new(), rule)
    }

    /// Depth-first search through the graph.
    /// Returns the number of valid paths which continue the given path.
    fn dfs(&self, curr_path: Path, rule: SmallCavesTwice) -> usize {
        if curr_path.is_finished() {
            return 1;
        }

        // Which nodes can we _not_ visit next?
        let mut exclude = HashSet::from([curr_path.curr_node.clone(), Cave::Start]);
        for (cave, num_visits) in &curr_path.small_cave_visits {
            let allowed = match rule {
                SmallCavesTwice::Never => 0,
                SmallCavesTwice::OnlyOnce if curr_path.has_visited_small_cave_more_than(1) => 0,
                _ => 1,
            };
            if num_visits > &allowed {
                exclude.insert(Cave::Small(cave.to_owned()));
            }
        }

        // Consider every possible next step -- which neighbouring caves could this path continue into?
        let neighbour_caves = self.edges.iter().flat_map(|(x, y)| {
            if x == &curr_path.curr_node || y == &curr_path.curr_node {
                vec![x, y]
            } else {
                vec![]
            }
        });
        let choices = neighbour_caves.filter(|cave| !exclude.contains(&cave));

        // What would the paths ahead look like, for each possible next path from here??
        choices
            .map(|choice| Path {
                curr_node: choice.clone(),
                small_cave_visits: if let Cave::Small(label) = choice {
                    let mut small_cave_vists = curr_path.small_cave_visits.clone();
                    *small_cave_vists.entry(label.to_owned()).or_insert(0) += 1;
                    small_cave_vists
                } else {
                    curr_path.small_cave_visits.clone()
                },
            })
            // Recursively count every path that continues from there
            .map(|next_path| self.dfs(next_path, rule))
            .sum()
    }
}

/// A path through the cave. This may be partial, or it may be finished.
struct Path {
    /// What is the current (latest) node along this path?
    curr_node: Cave,
    /// How many times has this path previously visited a given small cave?
    /// Missing key represents 0 times.
    small_cave_visits: HashMap<String, u8>,
}

impl Path {
    fn new() -> Self {
        Self {
            curr_node: Cave::Start,
            small_cave_visits: HashMap::new(),
        }
    }

    fn is_finished(&self) -> bool {
        self.curr_node == Cave::End
    }

    fn has_visited_small_cave_more_than(&self, freq: u8) -> bool {
        self.small_cave_visits.iter().any(|(_, v)| v > &freq)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tiny_q1() {
        let tests = [
            (10, include_str!("data/tiny.txt")),
            (19, include_str!("data/small.txt")),
            (226, include_str!("data/medium.txt")),
        ];
        for (expected, input) in tests {
            let g = Graph::parse_input(input);
            let actual = g.paths(SmallCavesTwice::Never);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_tiny_q2() {
        let tests = [
            (36, include_str!("data/tiny.txt")),
            (103, include_str!("data/small.txt")),
            (3509, include_str!("data/medium.txt")),
        ];
        for (expected, input) in tests {
            let g = Graph::parse_input(input);
            let actual = g.paths(SmallCavesTwice::OnlyOnce);
            assert_eq!(expected, actual);
        }
    }
}
