// Day 10 solution.

#![allow(dead_code)]

use std::env::args;
use std::fs;

struct Field {
    m: Vec<Vec<char>>,
    cols: usize,
    rows: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct Pt {
    x: i32,
    y: i32,
}

impl Pt {
    fn simplify(&self) -> Pt {
        // walk down from min(x,y) to 2, try to divide.
        for div in (2..=std::cmp::max(self.x.abs(), self.y.abs())).rev() {
            // dbg!(div);
            if (self.x % div == 0) && (self.y % div) == 0 {
                return Pt {
                    x: self.x / div,
                    y: self.y / div,
                };
            }
        }
        self.clone()
    }
}

impl std::ops::Add<&Pt> for &Pt {
    type Output = Pt;
    fn add(self, rhs: &Pt) -> Pt {
        Pt {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_simplify() {
        assert_eq!(Pt { x: 2, y: 14 }.simplify(), Pt { x: 1, y: 7 });
    }
    #[test]
    fn test_simplify0() {
        assert_eq!(Pt { x: 4, y: 0 }.simplify(), Pt { x: 1, y: 0 });
    }
    #[test]
    fn test_neg() {
        assert_eq!(Pt { x: 0, y: -2 }.simplify(), Pt { x: 0, y: -1 });
    }
}

struct FieldIt {
    cols: i32,
    rows: i32,
    x: i32,
    y: i32,
    done: bool,
}

impl Iterator for FieldIt {
    type Item = Pt;
    fn next(&mut self) -> Option<Pt> {
        if self.done {
            return None;
        }
        self.x += 1;
        if self.x >= self.cols {
            self.x = 0;
            self.y += 1;
        }
        if self.y >= self.rows {
            self.done = true;
            return None;
        }
        Some(Pt {
            x: self.x,
            y: self.y,
        })
    }
}

impl Field {
    fn parse_line(line: &str) -> Vec<char> {
        line.chars().collect()
    }

    fn iter(&self) -> FieldIt {
        FieldIt {
            cols: self.cols as i32,
            rows: self.rows as i32,
            x: 0,
            y: 0,
            done: false,
        }
    }

    fn has(&self, pt: &Pt) -> bool {
        self.m[pt.y as usize][pt.x as usize] == '#'
    }

    fn parse(file: &str) -> Field {
        let input = fs::read_to_string(file).unwrap();
        let mut lines = input.lines().peekable();
        let cols = lines.peek().unwrap().len();
        let m: Vec<Vec<char>> = lines.map(|line| Field::parse_line(line)).collect();
        let rows = m.len();
        Field { m, cols, rows }
    }

    fn is_visible_asteroid(&self, origin: &Pt, to: &Pt) -> bool {
        if !self.has(&to) {
            // If point has no asteroid, then we don't count it as visible.
            return false;
        }
        // Otherwise, we want to check whether it is occluded.
        let delta = Pt {
            x: origin.x - to.x,
            y: origin.y - to.y,
        }
        .simplify();
        let mut pt = to + &delta;
        // dbg!(origin, to, &delta, &pt);
        while pt != *origin {
            if self.has(&pt) {
                // Occluded.
                return false;
            }
            pt = &pt + &delta;
        }
        // visible
        true
    }

    fn count_visible_asteroids(&self, origin: &Pt) -> i32 {
        if !self.has(origin) {
            return 0;
        }
        let mut count = 0;
        for pt in self.iter() {
            if pt == *origin {
                continue;
            }
            let visible = self.is_visible_asteroid(origin, &pt);
            // dbg!(&pt, visible);
            if visible {
                count += 1;
            }
        }
        count
    }

    fn find_best_position(&self) -> i32 {
        let mut best = 0;
        let mut best_pos = None;
        for origin in self.iter() {
            let cnt = self.count_visible_asteroids(&origin);
            if cnt > best {
                best = cnt;
                best_pos = Some(origin);
            }
        }
        dbg!(best_pos);
        best
    }
}

fn main() {
    let filename = args().nth(1).expect("no filename given");
    let mut field = Field::parse(&filename);
    let origin = Pt { x: 5, y: 8 };
    // let pt = Pt { x: 4, y: 4 };
    // dbg!(&field.m);
    // dbg!(field.count_visible_asteroids(&origin));
    // dbg!(field.is_visible_asteroid(&origin, &pt));
    dbg!(field.find_best_position());
}
