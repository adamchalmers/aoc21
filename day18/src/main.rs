mod parse;
mod reduction;
mod sailfish_number;

use sailfish_number::Tree;

fn main() {
    let _homework =
        Tree::parse_many(include_str!("data/input.txt")).expect("could not parse input file");
}
