use std::cmp::{max, min};
use std::collections::HashSet;

#[derive(Debug)]
pub struct Cursor {
    x: i32,
    y: i32,

    // Keep track of points that have been visited. It's a pretty dum way to do this. If this
    // doesn't work, we'll track line segments instead.
    seen: HashSet<(i32, i32)>,

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
            seen: HashSet::new(),
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,
        }
    }

    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn go(&mut self, path: &Path, check_collisions: bool) -> i32 {
        let mut smol_dist = std::i32::MAX;
        for instr in path.0.iter() {
            let dist = self.apply(instr, check_collisions);
            smol_dist = min(smol_dist, dist);
        }
        smol_dist
    }

    fn apply(&mut self, instr: &PathInstruction, check_collisions: bool) -> i32 {
        let mut smol_dist = std::i32::MAX;
        let inc = match instr.dir {
            Dir::R => (1, 0),
            Dir::L => (-1, 0),
            Dir::U => (0, 1),
            Dir::D => (0, -1),
        };
        for _ in 0..instr.length {
            if check_collisions {
                if self.seen.contains(&(self.x, self.y)) {
                    // Manhattan distance.
                    let dist = self.x.abs() + self.y.abs();
                    smol_dist = min(smol_dist, dist);
                }
            } else {
                if self.x != 0 || self.y != 0 {
                    self.seen.insert((self.x, self.y));
                }
            }
            self.x += inc.0;
            self.y += inc.1;
        }
        self.min_x = min(self.min_x, self.x);
        self.max_x = max(self.max_x, self.x);
        self.min_y = min(self.min_y, self.y);
        self.max_y = max(self.max_y, self.y);
        smol_dist
    }

    pub fn go_string(&mut self, s: &str, check_collisions: bool) -> Option<i32> {
        let path = from_string(s);
        self.x = 0;
        self.y = 0;
        match self.go(&path, check_collisions) {
            std::i32::MAX => None,
            x => Some(x),
        }
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
        assert_eq!(c.go(&p, false), std::i32::MAX);
        assert_eq!(c.x, -1);
        assert_eq!(c.y, 2);
    }

    #[test]
    fn test_cross() {
        let mut c = Cursor::new();
        c.go_string("R75,D30,R83,U83,L12,D49,R71,U7,L72", false);
        assert_eq!(
            c.go_string("U62,R66,U55,R34,D71,R55,D58,R83", true)
                .unwrap(),
            159
        );
    }
}
