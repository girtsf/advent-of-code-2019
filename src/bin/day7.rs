// Day 7 runner.

#![allow(dead_code)]

extern crate adv_2019;

use adv_2019::intcode::State;
use std::env::args;

fn run(combo: &[i64], base: &State) -> i64 {
    let mut input = 0;
    for i in 0..5 {
        let mut state = base.clone();
        state.add_input(combo[i]);
        state.add_input(input);
        state.run(false);
        assert_eq!(state.outputs().len(), 1);
        input = state.outputs()[0];
    }
    // dbg!(input);
    input
}

fn main() {
    let filename = args().nth(1).expect("no filename given");
    let base_state = State::from_file(&filename);

    let mut best = 0;
    for q in 0..5 {
        for w in 0..5 {
            if q == w {
                continue;
            }
            for e in 0..5 {
                if q == e || w == e {
                    continue;
                }
                for r in 0..5 {
                    if q == r || w == r || e == r {
                        continue;
                    }
                    for t in 0..5 {
                        if q == t || w == t || e == t || r == t {
                            continue;
                        }
                        let val = run(&[q, w, e, r, t], &base_state);
                        if val > best {
                            best = val;
                            dbg!(val, q, w, e, r, t);
                        }
                    }
                }
            }
        }
    }
}
