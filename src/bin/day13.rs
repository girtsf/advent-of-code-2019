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

struct ScreenState {
    tiles: HashMap<Pos, Tile>,
}

impl ScreenState {
    fn new() -> Self {
        Self {
            tiles: HashMap::new(),
        }
    }

    fn paint(&mut self, x: i64, y: i64, tile_id: i64) {
        let tile = match tile_id {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("invalid tile_id"),
        };
        self.tiles.insert(Pos { x, y }, tile);
    }

    fn print_state(&self) {
        let min_x = self.tiles.iter().map(|(k, _)| k.x).min().unwrap();
        let max_x = self.tiles.iter().map(|(k, _)| k.x).max().unwrap();
        let min_y = self.tiles.iter().map(|(k, _)| k.y).min().unwrap();
        let max_y = self.tiles.iter().map(|(k, _)| k.y).max().unwrap();
        dbg!(min_x, max_x, min_y, max_y);
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
            break;
        }
    }
}

fn main() {
    let filename = args().nth(1).expect("no filename given");
    let intcode_state = State::from_file(&filename);
    let mut screen_state = ScreenState::new();
    screen_state.run(intcode_state);
    dbg!(&screen_state.block_count());
    screen_state.print_state();
}
