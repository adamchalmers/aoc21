use std::{num::ParseIntError, str::FromStr};

fn main() {
    let mut g = Grid::parse(include_str!("data/input.txt"));
    println!("Q1: {}", g.tick_n(100));
}

#[derive(Debug)]
struct Octopus {
    energy: u8,
    flashed: bool,
}

/// An Octopus can be parsed from a string containing exactly one digit, its energy.
impl FromStr for Octopus {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let energy = s.parse()?;
        Ok(Self {
            energy,
            flashed: false,
        })
    }
}

struct Point {
    x: usize,
    y: usize,
}

struct Grid<T> {
    x: usize,
    y: usize,
    cells: Vec<Vec<T>>,
}

impl<T> Grid<T>
where
    T: FromStr,
    <T as FromStr>::Err: core::fmt::Debug,
{
    fn parse(s: &str) -> Self {
        let cells: Vec<Vec<_>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| T::from_str(&String::from(char)).expect("could not parse a char"))
                    .collect()
            })
            .collect();

        let y = cells.len();
        let x = cells[0].len();
        Self { cells, x, y }
    }
}

impl<T> Grid<T> {
    fn neighbours(&self, p: Point) -> impl Iterator<Item = Point> {
        let width = self.x as isize;
        let height = self.y as isize;
        let x = p.x as isize;
        let y = p.y as isize;
        [
            (x + 1, y),
            (x - 1, y),
            (x, y + 1),
            (x, y - 1),
            (x + 1, y + 1),
            (x + 1, y - 1),
            (x - 1, y + 1),
            (x - 1, y - 1),
        ]
        .into_iter()
        .filter(move |(x, y)| x >= &0 && x < &width && y >= &0 && y < &height)
        .map(|(x, y)| Point {
            x: x as usize,
            y: y as usize,
        })
    }
    fn get(&self, x: usize, y: usize) -> &T {
        self.cells.get(x).unwrap().get(y).unwrap()
    }
    fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        self.cells.get_mut(x).unwrap().get_mut(y).unwrap()
    }
}

impl Grid<Octopus> {
    fn tick(&mut self) -> u16 {
        for x in 0..self.x {
            for y in 0..self.y {
                self.get_mut(x, y).flashed = false;
                self.get_mut(x, y).energy += 1;
            }
        }
        let mut num_flashes = 0;
        loop {
            let mut something_flashed = false;

            for x in 0..self.x {
                for y in 0..self.y {
                    if self.get(x, y).energy > 9 && !self.get(x, y).flashed {
                        self.get_mut(x, y).flashed = true;
                        num_flashes += 1;
                        something_flashed = true;
                        for p in self.neighbours(Point { x, y }) {
                            self.get_mut(p.x, p.y).energy += 1;
                        }
                    }
                }
            }

            if !something_flashed {
                break;
            }
        }

        for x in 0..self.x {
            for y in 0..self.y {
                if self.get(x, y).flashed {
                    self.get_mut(x, y).energy = 0;
                }
            }
        }
        num_flashes
    }
    fn tick_n(&mut self, n: usize) -> u16 {
        (0..n).fold(0, |num_flashes, _| num_flashes + self.tick())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbours() {
        let g = Grid::<Octopus>::parse(include_str!("data/example.txt"));
        assert_eq!(g.neighbours(Point { x: 0, y: 0 }).count(), 3);
        assert_eq!(g.neighbours(Point { x: 1, y: 1 }).count(), 8);
        assert_eq!(g.neighbours(Point { x: 0, y: 1 }).count(), 5);
    }

    #[test]
    fn test_one_flash() {
        let s = r#"11111
19991
19191
19991
11111"#;
        let mut g = Grid::<Octopus>::parse(s);
        assert_eq!(g.tick(), 9);
        assert_eq!(g.tick(), 0);
    }

    impl std::fmt::Debug for Grid<Octopus> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Grid")
                .field("x", &self.x)
                .field("y", &self.y)
                .finish()
        }
    }

    impl PartialEq for Octopus {
        fn eq(&self, other: &Self) -> bool {
            self.energy == other.energy
        }
    }

    impl Eq for Octopus {}

    impl Grid<Octopus> {
        fn print_cells(&self) {
            for y in 0..self.y {
                let mut line = String::new();
                for x in 0..self.x {
                    let ch = self.cells[x][y].energy.to_string().chars().next().unwrap();
                    line.push(ch)
                }
                println!("{}", line);
            }
        }
    }

    #[test]
    fn test_one_flash_example() {
        let mut g = Grid::<Octopus>::parse(include_str!("data/example.txt"));
        g.tick();
        let expected = Grid::<Octopus>::parse(include_str!("data/example_step2.txt"));
        g.print_cells();
        println!("\n");
        expected.print_cells();
        assert_eq!(g.cells, expected.cells);
    }

    #[test]
    fn test_example_q1() {
        let mut g = Grid::<Octopus>::parse(include_str!("data/example.txt"));
        let expected = 1656;
        let actual = g.tick_n(100);
        assert_eq!(actual, expected);
    }
}
