// Day 6 solution.

#![allow(dead_code)]

use std::collections::HashMap;
use std::fs;

struct Orbits {
    parents: HashMap<String, String>,
}

impl Orbits {
    fn parse(file: &str) -> Orbits {
        let input = fs::read_to_string(file).unwrap();
        let mut parents = HashMap::new();
        for line in input.lines() {
            assert_eq!(line.len(), 7);
            assert_eq!(line.chars().nth(3).unwrap(), ')');
            // We want to create links from rhs to lhs, so we add lhs to the list of rhs.
            let lhs = &line[0..3];
            let rhs = &line[4..7];
            assert!(!parents.contains_key(&rhs.to_string()));
            parents.insert(rhs.to_string(), lhs.to_string());
        }
        Orbits { parents }
    }

    fn count_up(&self, key: &str) -> i32 {
        let mut cnt = 0;
        let mut node = key;
        while self.parents.contains_key(node) {
            cnt += 1;
            node = self.parents.get(node).unwrap();
        }
        cnt
    }

    fn count_orbits(&self) -> i32 {
        let mut cnt = 0;
        for from in self.parents.keys() {
            dbg!(from);
            cnt += dbg!(self.count_up(from));
        }
        cnt
    }
}

fn main() {
    let orbits = Orbits::parse("inputs/day6.txt");
    dbg!(orbits.count_orbits());
}
