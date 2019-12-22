#![allow(dead_code)]

extern crate intcode;

use intcode::wires;
use std::fs;

fn main() {
    let contents = &fs::read_to_string("inputs/day3_1.txt").unwrap();
    let mut lines = contents.lines();
    let first_line = lines.next().unwrap();
    let second_line = lines.next().unwrap();
    dbg!(&first_line);
    dbg!(&second_line);
    let mut cursor = wires::Cursor::new();
    dbg!(cursor.pos());
    cursor.go_string(first_line, false);
    dbg!(cursor.pos());
    let distance = cursor.go_string(second_line, true).unwrap();
    dbg!(distance);
}
