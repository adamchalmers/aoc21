use std::{num::ParseIntError, str::FromStr};

const Q1_TURNS: usize = 100;

fn main() {
    let mut g = Grid::parse(include_str!("data/input.txt"));
    println!("Q1: {}", g.step_n(Q1_TURNS));
    println!("Q2: {}", g.synchronized_at() as usize + Q1_TURNS)
}

#[derive(Debug)]
struct Octopus {
    energy: u8,
    /// Did it flash in this step?
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

#[derive(Clone, Copy)]
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
    /// Parse the text files from Advent of Code.
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
    /// Find all neighbouring points.
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

    fn get(&self, p: Point) -> &T {
        self.cells.get(p.x).unwrap().get(p.y).unwrap()
    }

    fn get_mut(&mut self, p: Point) -> &mut T {
        self.cells.get_mut(p.x).unwrap().get_mut(p.y).unwrap()
    }

    /// Iterate over all points in the grid.
    fn points(&self) -> impl Iterator<Item = Point> {
        let height = self.y;
        let width = self.x;
        (0..height).flat_map(move |y| (0..width).map(move |x| Point { x, y }))
    }
}

impl Grid<Octopus> {
    /// Let n steps of time pass. Octopuses increase their energy and might flash.
    fn step_n(&mut self, n: usize) -> u16 {
        (0..n).fold(0, |num_flashes, _| num_flashes + self.step())
    }

    /// Let one step of time pass. Octopuses increase their energy and might flash.
    fn step(&mut self) -> u16 {
        // Each octopus gains energy.
        for p in self.points() {
            self.get_mut(p).flashed = false;
            self.get_mut(p).energy += 1;
        }

        // If any octopus flashes, its neighbours gain energy, possibly flashing themselves.
        let mut num_flashes = 0;
        loop {
            let mut something_flashed = false;

            for p in self.points() {
                if self.get(p).energy > 9 && !self.get(p).flashed {
                    self.get_mut(p).flashed = true;
                    num_flashes += 1;
                    something_flashed = true;
                    for p in self.neighbours(p) {
                        self.get_mut(p).energy += 1;
                    }
                }
            }

            if !something_flashed {
                break;
            }
        }

        // Reset the energy of any octopus that flashed in this step.
        for p in self.points() {
            if self.get(p).flashed {
                self.get_mut(p).energy = 0;
            }
        }
        num_flashes
    }

    /// Did every octopus flash in the previous step?
    fn all_flashed(&self) -> bool {
        self.points()
            .map(|p| self.get(p))
            .all(|octopus| octopus.flashed)
    }

    /// After how many steps will each octopus flash simultaneously?
    fn synchronized_at(&mut self) -> u16 {
        let mut i = 1;
        loop {
            self.step();
            if self.all_flashed() {
                return i;
            }
            i += 1;
        }
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
    fn test_one_flash_example() {
        let mut g = Grid::<Octopus>::parse(include_str!("data/example.txt"));
        g.step();
        let expected = Grid::<Octopus>::parse(include_str!("data/example_step2.txt"));
        assert_eq!(
            g.cells
                .into_iter()
                .map(|row| row.iter().map(|oct| oct.energy).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            expected
                .cells
                .into_iter()
                .map(|row| row.iter().map(|oct| oct.energy).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_example_q1() {
        let mut g = Grid::<Octopus>::parse(include_str!("data/example.txt"));
        let expected = 1656;
        let actual = g.step_n(100);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_example_q2() {
        let mut g = Grid::<Octopus>::parse(include_str!("data/example.txt"));
        let expected = 195;
        let actual = g.synchronized_at();
        assert_eq!(actual, expected);
    }
}
