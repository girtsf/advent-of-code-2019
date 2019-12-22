#![allow(dead_code)]

extern crate adv_2019;

use adv_2019::wires;
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
    cursor.go_string(second_line, true);
    dbg!(cursor.smol_manh_distance_collision());
    dbg!(cursor.least_steps_for_collision());
}
