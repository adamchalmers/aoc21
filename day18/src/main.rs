mod pair;
mod parse;

use pair::Pair;

fn main() {
    let _homework =
        Pair::parse_many(include_str!("data/input.txt")).expect("could not parse input file");
}
