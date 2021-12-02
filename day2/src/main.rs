fn stream_commands(s: &str) -> impl Iterator<Item = Command> + '_ {
    s.lines().map(Command::parse)
}

fn main() {
    let cmds = stream_commands(include_str!("input.txt"));
    let (q1_dst, q2_dst) = cmds.fold(
        // I'm applying the commands for Q1 and Q2 simultaneously so that I only need to iterate
        // over it once. This means I don't need to buffer it into memory or keep a copy. Just
        // stream it from disk.
        (Submarine::default(), Submarine::default()),
        |(q1_acc, q2_acc), cmd| (q1_acc.apply_q1(&cmd), q2_acc.apply_q2(&cmd)),
    );
    println!("Part 1 answer: {}", q1_dst.multiplied_distances());
    println!("Part 2 answer: {}", q2_dst.multiplied_distances());
}

enum Direction {
    Up,
    Down,
    Forward,
}

impl Direction {
    fn parse(s: &str) -> Self {
        match s {
            "forward" => Self::Forward,
            "down" => Self::Down,
            "up" => Self::Up,
            other => unreachable!("unexpected direction {}", other),
        }
    }
}

struct Command {
    direction: Direction,
    distance: u32,
}

impl Command {
    fn parse(s: &str) -> Self {
        let mut parts = s.split(' ');
        let direction = Direction::parse(parts.next().unwrap());
        let distance = parts.next().unwrap().parse().unwrap();
        Self {
            direction,
            distance,
        }
    }
}

#[derive(Default)]

struct Submarine {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

impl Submarine {
    /// Apply the command to the submarine, based on the rules from q1
    fn apply_q1(mut self, cmd: &Command) -> Self {
        match cmd.direction {
            Direction::Down => self.depth += cmd.distance,
            Direction::Up => self.depth -= cmd.distance,
            Direction::Forward => self.horizontal += cmd.distance,
        }
        self
    }

    /// Apply the command to the submarine, based on the rules from q2
    fn apply_q2(mut self, cmd: &Command) -> Self {
        match cmd.direction {
            Direction::Down => self.aim += cmd.distance,
            Direction::Up => self.aim -= cmd.distance,
            Direction::Forward => {
                self.horizontal += cmd.distance;
                self.depth += cmd.distance * self.aim;
            }
        }
        self
    }

    fn multiplied_distances(&self) -> u32 {
        self.horizontal * self.depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q1() {
        let cmds = stream_commands(include_str!("example_input.txt"));
        let destination = cmds.fold(Submarine::default(), |sub, cmd| sub.apply_q1(&cmd));
        assert_eq!(150, destination.multiplied_distances())
    }

    #[test]
    fn test_q2() {
        let cmds = stream_commands(include_str!("example_input.txt"));
        let destination = cmds.fold(Submarine::default(), |sub, cmd| sub.apply_q2(&cmd));
        assert_eq!(900, destination.multiplied_distances())
    }
}
