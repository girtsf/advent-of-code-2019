#![allow(dead_code)]

use std::env::args;

mod intcode;

fn main() {
    // intcode::day2_part2();
    let filename = args().nth(1).expect("no filename given");
    let mut state = intcode::State::from_file(&filename);
    state.run(true);
}
