#![allow(dead_code)]

extern crate intcode;

use intcode::intcode::State;
use std::env::args;

fn main() {
    // intcode::day2_part2();
    let filename = args().nth(1).expect("no filename given");
    let mut state = State::from_file(&filename);
    state.run(true);
}
