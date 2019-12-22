#![allow(dead_code)]

extern crate adv_2019;

use adv_2019::intcode::State;
use std::env::args;

fn main() {
    // intcode::day2_part2();
    let filename = args().nth(1).expect("no filename given");
    let mut state = State::from_file(&filename);
    state.run(true);
}
