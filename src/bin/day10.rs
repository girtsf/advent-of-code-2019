// Day 10 solution.

#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::BTreeMap;
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

// Pt, sortable and comparable by angle.
#[derive(Debug, Clone, PartialEq)]
struct AnglePt(Pt);

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

impl AnglePt {
    pub fn new(p: &Pt) -> Self {
        Self(p.simplify())
    }

    fn sector(&self) -> i32 {
        let Pt { x, y } = self.0;
        assert!(x != 0 || y != 0);
        if x == 0 && y < 0 {
            1
        } else if x > 0 {
            if y < 0 {
                2
            } else if y == 0 {
                3
            } else {
                4
            }
        } else if x == 0 && y > 0 {
            5
        } else {
            if y > 0 {
                6
            } else if y == 0 {
                7
            } else {
                8
            }
        }
    }
}

// Tell the compiler that our PartialEq is Eq.
impl Eq for AnglePt {}

impl PartialOrd for AnglePt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AnglePt {
    fn cmp(&self, other: &Self) -> Ordering {
        let s1 = self.sector();
        let s2 = other.sector();
        if s1 < s2 {
            Ordering::Less
        } else if s1 > s2 {
            Ordering::Greater
        } else {
            if self == other {
                Ordering::Equal
            } else {
                // We are in the same sector. Compare the slopes.
                // slope_self = self.y / self.x
                // slope_other = other.y / other.x
                // slope_self = (self.y * other.x) / (self.x * other.x)
                // slope_other = (other.y * self.x) / (self.x * other.x)
                // slope_self <=> slope.other is same as (self.y * other.x) <=> (other.y * self.x)
                (self.0.y * other.0.x).cmp(&(other.0.y * self.0.x))
            }
        }
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

impl std::ops::Sub<&Pt> for &Pt {
    type Output = Pt;
    fn sub(self, rhs: &Pt) -> Pt {
        Pt {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
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
    #[test]
    fn test_angle_pt() {
        assert_eq!(
            AnglePt::new(&Pt { x: 1, y: 0 }),
            AnglePt::new(&Pt { x: 1, y: 0 })
        );
        assert_eq!(
            AnglePt::new(&Pt { x: 5, y: -2 }),
            AnglePt::new(&Pt { x: 10, y: -4 })
        );
        assert!(AnglePt::new(&Pt { x: 1, y: -2 }) < AnglePt::new(&Pt { x: 2, y: -2 }));
        assert!(AnglePt::new(&Pt { x: 2, y: -2 }) > AnglePt::new(&Pt { x: 1, y: -2 }));
        let pt = AnglePt::new(&Pt { x: 5, y: -2 });
        assert_eq!(pt, pt.clone());
        assert_eq!(AnglePt::new(&Pt { x: -1, y: -1 }).sector(), 8);
        assert_eq!(AnglePt::new(&Pt { x: 0, y: -1 }).sector(), 1);
        assert!(AnglePt::new(&Pt { x: -1, y: -1 }) > AnglePt::new(&Pt { x: 0, y: -1 }));
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
        let delta = (origin - to).simplify();
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

    fn find_best_position(&self) -> (i32, Pt) {
        let mut best = 0;
        let mut best_pos = None;
        for origin in self.iter() {
            let cnt = self.count_visible_asteroids(&origin);
            if cnt > best {
                best = cnt;
                best_pos = Some(origin);
            }
        }
        (best, best_pos.unwrap())
    }

    fn sort_by_angle(&self, origin: &Pt) -> BTreeMap<AnglePt, Vec<Pt>> {
        let mut map = BTreeMap::new();
        for pt in self.iter() {
            if pt == *origin {
                continue;
            }
            if !self.has(&pt) {
                continue;
            }
            let delta = &pt - origin;
            let angle = AnglePt::new(&delta);
            if !map.contains_key(&angle) {
                map.insert(angle.clone(), Vec::new());
            }
            map.get_mut(&angle).unwrap().push(delta);
        }
        // Sort vector in each angle.
        for v in map.values_mut() {
            v.sort_by(|a, b| (a.x * a.x + a.y * a.y).cmp(&(b.x * b.x + b.y * b.y)));
        }
        map
    }

    fn vaporize(&self, origin: &Pt) -> Pt {
        let mut map = self.sort_by_angle(origin);
        let mut i = 0;
        loop {
            for v in map.values_mut() {
                if v.is_empty() {
                    continue;
                }
                let first = v.remove(0); // results in O(N^2)
                i += 1;
                // println!(
                //     "vaporizing {}: relative: {:?}, absolute: {:?}",
                //     i,
                //     first,
                //     (&first + origin)
                // );
                if i == 200 {
                    return &first + origin;
                }
            }
        }
    }
}

fn main() {
    let filename = args().nth(1).expect("no filename given");
    let mut field = Field::parse(&filename);
    // let origin = Pt { x: 11, y: 13 };
    // let pt = Pt { x: 4, y: 4 };
    // dbg!(&field.m);
    // dbg!(field.count_visible_asteroids(&origin));
    // dbg!(field.is_visible_asteroid(&origin, &pt));
    let (_, origin) = dbg!(field.find_best_position());
    let vaporized_pt = dbg!(field.vaporize(&origin));
    dbg!(vaporized_pt.x * 100 + vaporized_pt.y);
}
