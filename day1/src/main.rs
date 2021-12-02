type Depth = u16;

fn main() {
    let depths = read(include_str!("input.txt"));
    let part1 = count_increases(depths.iter());
    println!("Part 1 answer: {}", part1);

    let window_depths = depths.windows(3).map(|w| w.iter().sum::<u16>());
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

/// Read a newline-separated list of Depths.
fn read(file: &str) -> Vec<Depth> {
    file.lines().map(|s| s.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let depths_example = read(include_str!("example.txt"));
        assert_eq!(count_increases(depths_example.into_iter()), 7);
    }

    #[test]
    fn test_part_2() {
        let depths_example = read(include_str!("example.txt"));
        let depths_example_window = depths_example.windows(3).map(|w| w.iter().sum::<Depth>());
        assert_eq!(count_increases(depths_example_window), 5);
    }
}
