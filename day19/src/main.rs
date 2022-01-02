#![feature(generic_const_exprs)]
#![feature(const_fn_trait_bound)]
mod parse;
mod rotations;
mod solve;

fn main() {
    let _problem = parse::Problem::parse(include_str!("data/input.txt")).unwrap();
}
