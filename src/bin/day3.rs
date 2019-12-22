#![allow(dead_code)]

extern crate intcode;

use intcode::wires;
use std::fs;

fn main() {
    let contents = &fs::read_to_string("inputs/day3_1.txt").unwrap();
    let first_line = contents.lines().next().unwrap();
    dbg!(first_line);
    let path = wires::from_string(first_line);
    let mut cursor = wires::Cursor::new();
    cursor.go(&path);
    dbg!(cursor);
}
