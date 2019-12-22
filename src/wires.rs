use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Cursor {
    x: i32,
    y: i32,

    // Keep track of points that have been visited. Values are number of steps we took to get
    // there.
    seen: HashMap<(i32, i32), i32>,

    // Steps taken so far.
    steps: i32,

    // When checking for collisions, this gets updated to the smallest Manhattan distance to the
    // origin.
    smol_manh_distance_collision: i32,

    // When checking for collisions, this gets updated with least steps to get a collision.
    least_steps_for_collision: i32,

    // Bounding box.
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

/// Direction of a path instruction.
#[derive(Debug, PartialEq)]
enum Dir {
    R,
    L,
    U,
    D,
}

/// Describes one instruction in the path, e.g., "R995".
struct PathInstruction {
    dir: Dir,
    length: i32,
}

pub struct Path(Vec<PathInstruction>);

impl Cursor {
    pub fn new() -> Cursor {
        Cursor {
            x: 0,
            y: 0,
            seen: HashMap::new(),
            steps: 0,
            smol_manh_distance_collision: std::i32::MAX,
            least_steps_for_collision: std::i32::MAX,
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,
        }
    }

    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn smol_manh_distance_collision(&self) -> Option<i32> {
        if self.smol_manh_distance_collision != std::i32::MAX {
            Some(self.smol_manh_distance_collision)
        } else {
            None
        }
    }

    pub fn least_steps_for_collision(&self) -> Option<i32> {
        if self.least_steps_for_collision != std::i32::MAX {
            Some(self.least_steps_for_collision)
        } else {
            None
        }
    }

    pub fn go(&mut self, path: &Path, check_collisions: bool) {
        for instr in path.0.iter() {
            self.apply(instr, check_collisions);
        }
    }

    fn apply(&mut self, instr: &PathInstruction, check_collisions: bool) {
        let inc = match instr.dir {
            Dir::R => (1, 0),
            Dir::L => (-1, 0),
            Dir::U => (0, 1),
            Dir::D => (0, -1),
        };
        for _ in 0..instr.length {
            if check_collisions {
                if let Some(previous_steps) = self.seen.get(&self.pos()) {
                    // Calculate Manhattan distance for part 1.
                    let dist = self.x.abs() + self.y.abs();
                    self.smol_manh_distance_collision =
                        min(self.smol_manh_distance_collision, dist);
                    // Calculate steps for part 2.
                    let steps = previous_steps + self.steps;
                    dbg!(steps);
                    self.least_steps_for_collision = min(self.least_steps_for_collision, steps);
                }
            } else {
                // Don't insert in pos (0, 0) as we don't check for collisions there.
                if self.x != 0 || self.y != 0 {
                    // Only insert the smallest.
                    if !self.seen.contains_key(&(self.pos())) {
                        self.seen.insert(self.pos(), self.steps);
                    }
                }
            }
            self.x += inc.0;
            self.y += inc.1;
            self.steps += 1;
        }
        self.min_x = min(self.min_x, self.x);
        self.max_x = max(self.max_x, self.x);
        self.min_y = min(self.min_y, self.y);
        self.max_y = max(self.max_y, self.y);
    }

    pub fn go_string(&mut self, s: &str, check_collisions: bool) {
        let path = from_string(s);
        self.x = 0;
        self.y = 0;
        self.steps = 0;
        self.go(&path, check_collisions);
    }
}

impl PathInstruction {
    /// Parses one "R995".
    fn parse(s: &[u8]) -> PathInstruction {
        let dir = match s[0] {
            b'R' => Dir::R,
            b'L' => Dir::L,
            b'U' => Dir::U,
            b'D' => Dir::D,
            _ => panic!("invalid char: {}", s[0]),
        };
        let length = std::str::from_utf8(&s[1..])
            .unwrap()
            .parse::<i32>()
            .unwrap();
        PathInstruction { dir, length }
    }
}

pub fn from_string(s: &str) -> Path {
    Path(
        s.trim()
            .split(',')
            .map(|x| PathInstruction::parse(x.as_bytes()))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pathinstruction() {
        let p = PathInstruction::parse("L123".as_bytes());
        assert_eq!(p.dir, Dir::L);
        assert_eq!(p.length, 123);
    }

    #[test]
    fn test_from_string() {
        let p = from_string("L123,R444");
        assert_eq!(p.0.len(), 2);
        assert_eq!(p.0[0].dir, Dir::L);
        assert_eq!(p.0[0].length, 123);
    }

    #[test]
    fn test_apply() {
        let mut c = Cursor::new();
        let p = from_string("L2,U3,R1,D1");
        c.go(&p, false);
        assert_eq!(c.x, -1);
        assert_eq!(c.y, 2);
    }

    #[test]
    fn test_part1() {
        let mut c = Cursor::new();
        c.go_string("R75,D30,R83,U83,L12,D49,R71,U7,L72", false);
        c.go_string("U62,R66,U55,R34,D71,R55,D58,R83", true);
        assert_eq!(c.smol_manh_distance_collision().unwrap(), 159);
    }

    #[test]
    fn test_part2_simple() {
        let mut c = Cursor::new();
        c.go_string("R8,U5,L5,D3", false);
        dbg!(&c.seen);
        c.go_string("U7,R6,D4,L4", true);
        assert_eq!(c.least_steps_for_collision().unwrap(), 30);
    }

    #[test]
    fn test_part2() {
        let mut c = Cursor::new();
        c.go_string("R75,D30,R83,U83,L12,D49,R71,U7,L72", false);
        dbg!(&c.seen);
        c.go_string("U62,R66,U55,R34,D71,R55,D58,R83", true);
        assert_eq!(c.least_steps_for_collision().unwrap(), 610);
    }
}
