type Quantity = u64;

/// A school of fish, grouped by days until they next reproduce.
/// Each index is the quantity of fish with that many days until they next reproduce.
type School = [Quantity; 9];

fn main() {
    let fish = parse_problem(include_str!("input.txt"));
    let fish_80 = simulate(fish, 80);
    let q1: Quantity = fish_80.iter().sum();
    println!("Q1: {}", q1);
    let fish_256 = simulate(fish_80, 256 - 80);
    let q2: Quantity = fish_256.iter().sum();
    println!("Q2: {}", q2);
}

fn parse_problem(s: &str) -> School {
    let mut school = [0; 9];
    for fish in s.split(',') {
        let days_until_reproduction: usize = fish.parse().unwrap();
        school[days_until_reproduction] += 1;
    }
    school
}

fn simulate(mut fish: School, days: usize) -> School {
    for _ in 0..days {
        let mut new_fish = [0; 9];
        // Each of the fish age by a day.
        new_fish[..8].clone_from_slice(&fish[1..(8 + 1)]);
        new_fish[6] += fish[0];
        // And new fish spawn.
        new_fish[8] = fish[0];
        fish = new_fish;
    }
    fish
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_small() {
        let input = parse_problem(include_str!("example.txt"));
        assert_eq!(simulate(input, 18).into_iter().sum::<Quantity>(), 26);
    }

    #[test]
    fn test_example() {
        let input = parse_problem(include_str!("example.txt"));
        assert_eq!(simulate(input, 80).into_iter().sum::<Quantity>(), 5934);
    }
}
