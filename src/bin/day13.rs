// Day 13 runner.

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
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

#[derive(Default)]
struct ScreenState {
    tiles: HashMap<Pos, Tile>,
    score: i64,
    paddle: i64,
    ball: i64,
}

impl ScreenState {
    fn new() -> Self {
        Self {
            tiles: HashMap::new(),
            ..Default::default()
        }
    }

    fn paint(&mut self, x: i64, y: i64, tile_id: i64) {
        if x == -1 && y == 0 {
            self.score = tile_id;
            return;
        }
        let tile = match tile_id {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => {
                self.paddle = x;
                Tile::Paddle
            }
            4 => {
                self.ball = x;
                Tile::Ball
            }
            _ => panic!("invalid tile_id"),
        };
        // if tile == Tile::Paddle {
        // }
        self.tiles.insert(Pos { x, y }, tile);
    }

    fn print_state(&self) {
        dbg!(&self.score);
        dbg!(&self.paddle);
        let min_x = self.tiles.iter().map(|(k, _)| k.x).min().unwrap();
        let max_x = self.tiles.iter().map(|(k, _)| k.x).max().unwrap();
        let min_y = self.tiles.iter().map(|(k, _)| k.y).min().unwrap();
        let max_y = self.tiles.iter().map(|(k, _)| k.y).max().unwrap();
        // dbg!(min_x, max_x, min_y, max_y);
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if let Some(c) = self.tiles.get(&Pos { x, y }) {
                    let ch = match c {
                        Tile::Wall => '#',
                        Tile::Block => 'x',
                        Tile::Paddle => '_',
                        Tile::Ball => 'o',
                        _ => ' ',
                    };
                    print!("{}", ch);
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    fn block_count(&self) -> usize {
        self.tiles
            .iter()
            .fold(0, |cnt, (_, v)| cnt + if *v == Tile::Block { 1 } else { 0 })
    }

    fn run(&mut self, mut state: State) {
        loop {
            // state.add_input(self.get_color_as_int(&self.pos));
            let stop_reason = state.run(false);
            // assert_eq!(stop_reason, StopReason::Done);
            let outputs = state.outputs();
            while !outputs.is_empty() {
                let x = outputs.pop_front().unwrap();
                let y = outputs.pop_front().unwrap();
                let tile_id = outputs.pop_front().unwrap();
                self.paint(x, y, tile_id);
            }
            match stop_reason {
                StopReason::WaitingOnInput => {
                    let joystick = if self.ball < self.paddle {
                        -1
                    } else if self.ball > self.paddle {
                        1
                    } else {
                        0
                    };
                    state.add_input(joystick);
                }
                StopReason::Done => break,
            }
            // self.print_state();
        }
        self.print_state();
    }
}

fn main() {
    let filename = args().nth(1).expect("no filename given");
    let mut intcode_state = State::from_file(&filename);
    // Insert a quarter.
    intcode_state.write(0, 2);
    let mut screen_state = ScreenState::new();
    screen_state.run(intcode_state);
    // dbg!(&screen_state.block_count());
}
