// Day 7 runner.

#![allow(dead_code)]

extern crate adv_2019;

use adv_2019::intcode::{State, StopReason};
use std::env::args;

fn run(combo: &[i64], base: &State) -> i64 {
    let mut states = Vec::new();
    for i in 0..5 {
        let mut state = base.clone();
        state.add_input(combo[i]);
        if i == 0 {
            state.add_input(0);
        }
        states.push(state);
    }
    let mut to_run = 0;
    loop {
        let stop_reason = states[to_run].run(false);
        // dbg!(&to_run, &stop_reason);
        match stop_reason {
            StopReason::Done => {
                if to_run == 4 {
                    let outputs = states[to_run].outputs();
                    assert_eq!(outputs.len(), 1);
                    return outputs.pop_front().unwrap();
                }
            }
            _ => {}
        };
        let next = (to_run + 1) % 5;
        while let Some(value) = states[to_run].outputs().pop_front() {
            states[next].add_input(value);
        }
        to_run = next;
    }
}

fn main() {
    let filename = args().nth(1).expect("no filename given");
    let base_state = State::from_file(&filename);
    let phase_min = args()
        .nth(2)
        .expect("no phase start given")
        .parse::<i64>()
        .unwrap();
    let phases = phase_min..(phase_min + 5);

    let mut best = 0;
    for q in phases.clone() {
        for w in phases.clone() {
            if q == w {
                continue;
            }
            for e in phases.clone() {
                if q == e || w == e {
                    continue;
                }
                for r in phases.clone() {
                    if q == r || w == r || e == r {
                        continue;
                    }
                    for t in phases.clone() {
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
