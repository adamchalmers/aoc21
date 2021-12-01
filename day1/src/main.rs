use std::collections::VecDeque;

type Depth = u16;

fn main() {
    let depths = read(include_str!("input.txt"));
    let part1 = count_increases(depths.clone().into_iter());
    println!("Part 1 answer: {}", part1);

    let window_depths = Window::new(depths, 3).unwrap();
    let part2 = count_increases(window_depths);
    println!("Part 2 answer: {}", part2);
}

/// How many times does this iterator increase from one item to the next?
fn count_increases<T: Ord, Iter: Iterator<Item = T>>(mut iter: Iter) -> u16 {
    let mut increases = 0;
    let mut curr = iter.next().unwrap();
    for next in iter {
        if next > curr {
            increases += 1;
        }
        curr = next;
    }
    increases
}

/// Scans over the vector with a window of the given length.
/// When there are no more elements left, it will yield smaller and smaller windows.
struct Window {
    curr: VecDeque<Depth>,
    remaining: Vec<Depth>,
    max_length: usize,
}

impl Window {
    fn new(mut vec: Vec<Depth>, max_length: usize) -> Option<Self> {
        vec.reverse(); // This way, pop removes items from the start
        let mut curr = VecDeque::new();
        for _ in 1..max_length {
            curr.push_front(vec.pop()?);
        }
        Some(Self {
            curr,
            remaining: vec,
            max_length,
        })
    }
}

impl Iterator for Window {
    type Item = Depth;
    fn next(&mut self) -> Option<Self::Item> {
        match self.remaining.pop() {
            Some(next) => {
                // Make sure the window never exceeds the maximum length.
                if self.curr.len() == self.max_length {
                    self.curr.pop_back()?;
                }
                self.curr.push_front(next);

                Some(self.curr.iter().sum())
            }
            None => {
                // Shrink the window, sum any items left in it.
                self.curr.pop_back()?;
                if self.curr.is_empty() {
                    None
                } else {
                    Some(self.curr.iter().sum())
                }
            }
        }
    }
}

/// Read a newline-separated list of Depths.
fn read(file: &str) -> Vec<Depth> {
    file.split('\n').map(|s| s.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window() {
        let mut window = Window::new(vec![1, 2, 3, 4], 2).unwrap();
        assert_eq!(window.next(), Some(3));
        assert_eq!(window.next(), Some(5));
        assert_eq!(window.next(), Some(7));
        assert_eq!(window.next(), Some(4));
        assert_eq!(window.next(), None);
    }

    #[test]
    fn test_part_1() {
        let depths_example = read(include_str!("example.txt"));
        assert_eq!(count_increases(depths_example.into_iter()), 7);
    }

    #[test]
    fn test_part_2() {
        let depths_example = read(include_str!("example.txt"));
        let depths_example_window = Window::new(depths_example, 3).unwrap();
        assert_eq!(count_increases(depths_example_window), 5);
    }
}
