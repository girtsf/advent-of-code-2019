#[derive(Debug)]
pub struct Cursor {
    x: i32,
    y: i32,
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
        Cursor { x: 0, y: 0 }
    }

    pub fn go(&mut self, path: &Path) {
        for instr in path.0.iter() {
            self.apply(instr);
        }
    }

    fn apply(&mut self, instr: &PathInstruction) {
        let multipliers = match instr.dir {
            Dir::R => (1, 0),
            Dir::L => (-1, 0),
            Dir::U => (0, 1),
            Dir::D => (0, -1),
        };
        self.x += instr.length * multipliers.0;
        self.y += instr.length * multipliers.1;
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
        c.go(&p);
        assert_eq!(c.x, -1);
        assert_eq!(c.y, 2);
    }
}
