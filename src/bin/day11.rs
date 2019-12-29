// Day 7 runner.

#![allow(dead_code)]

extern crate adv_2019;

use adv_2019::intcode::{State, StopReason};
use std::collections::HashMap;
use std::env::args;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Pos {
    x: i64,
    y: i64,
}

#[derive(PartialEq)]
enum Color {
    Black,
    White,
}

impl Color {
    fn from_int(i: i64) -> Self {
        match i {
            0 => Self::Black,
            1 => Self::White,
            _ => panic!("invalid color: {}", i),
        }
    }
    fn to_int(&self) -> i64 {
        match self {
            Self::Black => 0,
            Self::White => 1,
        }
    }
}

enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    fn turn_right(&self) -> Self {
        match self {
            Self::U => Self::R,
            Self::R => Self::D,
            Self::D => Self::L,
            Self::L => Self::U,
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Self::U => Self::L,
            Self::L => Self::D,
            Self::D => Self::R,
            Self::R => Self::U,
        }
    }

    fn d(&self) -> (i64, i64) {
        match self {
            Self::U => (0, -1),
            Self::R => (1, 0),
            Self::D => (0, 1),
            Self::L => (-1, 0),
        }
    }
}

struct PaintingState {
    panels: HashMap<Pos, Color>,
    pos: Pos,
    dir: Dir,
}

impl Pos {
    fn new() -> Pos {
        Pos { x: 0, y: 0 }
    }

    fn mv(&mut self, dir: &Dir) -> Pos {
        let d = dir.d();
        Pos {
            x: self.x + d.0,
            y: self.y + d.1,
        }
    }
}

impl PaintingState {
    fn new() -> Self {
        Self {
            panels: HashMap::new(),
            pos: Pos::new(),
            dir: Dir::U,
        }
    }

    fn get_color_as_int(&self, pos: &Pos) -> i64 {
        match self.panels.get(pos) {
            Some(c) => c.to_int(),
            None => 0,
        }
    }

    fn run(&mut self, mut state: State) {
        loop {
            state.add_input(self.get_color_as_int(&self.pos));
            let stop_reason = state.run(false);
            let outputs = state.outputs();
            while !outputs.is_empty() {
                let color_to_paint = outputs.pop_front().unwrap();
                self.panels
                    .insert(self.pos.clone(), Color::from_int(color_to_paint));

                let dir_to_turn = outputs.pop_front().unwrap();
                match dir_to_turn {
                    0 => self.dir = self.dir.turn_left(),
                    1 => self.dir = self.dir.turn_right(),
                    _ => panic!("invalid dir: {}", dir_to_turn),
                }
                self.pos = self.pos.mv(&self.dir);
            }
            match stop_reason {
                StopReason::Done => {
                    return;
                }
                StopReason::WaitingOnInput => {}
            }
        }
    }

    fn count_colored_panels(&self) -> i64 {
        return self.panels.len() as i64;
    }

    fn print_state(&self) {
        let min_x = self.panels.iter().map(|(k, _)| k.x).min().unwrap();
        let max_x = self.panels.iter().map(|(k, _)| k.x).max().unwrap();
        let min_y = self.panels.iter().map(|(k, _)| k.y).min().unwrap();
        let max_y = self.panels.iter().map(|(k, _)| k.y).max().unwrap();
        dbg!(min_x, max_x, min_y, max_y);
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if let Some(c) = self.panels.get(&Pos { x, y }) {
                    if *c == Color::Black {
                        print!(" ");
                    } else {
                        print!("#");
                    }
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

fn main() {
    let filename = args().nth(1).expect("no filename given");
    let state = State::from_file(&filename);
    let mut painting_state = PaintingState::new();
    // For part2:
    painting_state
        .panels
        .insert(Pos { x: 0, y: 0 }, Color::White);
    painting_state.run(state);
    dbg!(painting_state.count_colored_panels());
    painting_state.print_state();
}
