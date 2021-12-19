mod parse;
mod sailfish_number;

use sailfish_number::Number;

fn main() {
    let _homework =
        Number::parse_many(include_str!("data/input.txt")).expect("could not parse input file");
}
