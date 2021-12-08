use std::collections::HashSet;

const BINGO_SIZE: usize = 5;

/// A standard Bingo board with N columns and N rows.
struct Board<T, const N: usize>([[T; N]; N]);

impl<const N: usize> Board<u8, N> {
    fn parse(lines: [&str; N]) -> Self {
        Self(lines.map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        }))
    }

    /// Is this board a winning Bingo?
    fn is_winner(&self, drawn: &HashSet<u8>) -> bool {
        let rows_and_columns: Vec<HashSet<_>> = (0..N)
            .flat_map(|i| {
                [
                    (0..N).map(|j| self.0[j][i]).collect(), // Rows
                    (0..N).map(|j| self.0[i][j]).collect(), // Columns
                ]
            })
            .collect();
        for row_or_column in rows_and_columns {
            if row_or_column.is_subset(drawn) {
                return true;
            }
        }
        false
    }

    fn unmarked<'a>(&'a self, drawn: &'a HashSet<u8>) -> impl Iterator<Item = u8> + 'a {
        (0..N * N)
            .map(|n| self.0[n / N][n % N])
            .filter(|n| !drawn.contains(n))
    }
}

struct BingoGame {
    /// Remaining bingo numbers, in the reverse order they will be drawn.
    draws: Vec<u8>,
    /// Available boards for players
    boards: Vec<Board<u8, BINGO_SIZE>>,
    /// Numbers already drawn
    seen: HashSet<u8>,
}

impl BingoGame {
    fn parse(s: &str) -> Self {
        // Get the list of numbers
        let mut lines = s.lines();
        let mut draws: Vec<u8> = lines
            .next()
            .unwrap()
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect();
        draws.reverse();
        lines.next();

        // Get the boards by reading batches of lines.
        let boards: BatchIterator<&str, _, BINGO_SIZE> = BatchIterator {
            inner_iterator: lines.into_iter().filter(|l| !l.is_empty()),
        };
        Self {
            draws,
            boards: boards.into_iter().map(Board::parse).collect(),
            seen: Default::default(),
        }
    }

    fn play_one_round(&mut self) -> u8 {
        let number_drawn = self.draws.pop().expect("Game is over");
        self.seen.insert(number_drawn);
        number_drawn
    }

    fn play(mut self) -> (u32, u32) {
        let mut first_winner = 0;
        let mut winning_boards = HashSet::new();

        loop {
            // Each iteration is one round of the game.
            let number_drawn = self.play_one_round() as u32;

            // Check if any boards won.
            for (board_num, board) in self.boards.iter().enumerate() {
                if !winning_boards.contains(&board_num) && board.is_winner(&self.seen) {
                    let score: u32 =
                        board.unmarked(&self.seen).map(|n| n as u32).sum::<u32>() * number_drawn;
                    if first_winner == 0 {
                        first_winner = score
                    }
                    winning_boards.insert(board_num);
                    if winning_boards.len() == self.boards.len() {
                        return (first_winner, score);
                    }
                }
            }
        }
    }
}

/// An iterator adaptor. It takes an iterator, and returns N items at a time.
struct BatchIterator<T, I: Iterator<Item = T>, const N: usize> {
    inner_iterator: I,
}

impl<T, I: Iterator<Item = T>, const N: usize> Iterator for BatchIterator<T, I, N>
where
    T: Default,
{
    type Item = [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        let line: Option<Vec<T>> = (0..N).map(|_| self.inner_iterator.next()).collect();
        let line = line?.try_into().ok()?;
        Some(line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let game = BingoGame::parse(include_str!("example.txt"));
        assert_eq!(game.boards.len(), 3);
        assert_eq!(game.draws.len(), 27);
        let (first_winner, last_winner) = game.play();
        assert_eq!(first_winner, 4512);
        assert_eq!(last_winner, 1924);
    }
}

fn main() {
    let game = BingoGame::parse(include_str!("input.txt"));
    println!(
        "Loaded {} boards and {} numbers",
        game.boards.len(),
        game.draws.len()
    );
    let (first_winner, last_winner) = game.play();
    println!("Q1: {}", first_winner);
    println!("Q2: {}", last_winner);
}
