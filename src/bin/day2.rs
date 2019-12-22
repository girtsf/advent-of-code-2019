// Day 2 and day 5 runner.

#![allow(dead_code)]

extern crate adv_2019;

use adv_2019::intcode::State;
use std::env::args;

fn main() {
    // intcode::day2_part2();
    let filename = args().nth(1).expect("no filename given");
    let mut state = State::from_file(&filename);
    for arg in args().skip(2) {
        state.add_input(arg.parse::<i32>().unwrap());
    }
    state.run(true);
    dbg!(state.outputs());
}
