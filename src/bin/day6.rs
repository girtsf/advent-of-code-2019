// Day 6 solution.

#![allow(dead_code)]

use std::collections::HashMap;
use std::fs;

struct Orbits {
    parents: HashMap<String, String>,
    distance_to_san: HashMap<String, i32>,
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
        Orbits {
            parents,
            distance_to_san: HashMap::new(),
        }
    }

    fn mark_distances_to_san(&mut self) {
        let mut cnt = 0;
        let mut node = self.parents.get("SAN").unwrap();
        while self.parents.contains_key(node) {
            self.distance_to_san.insert(node.to_string(), cnt);
            cnt += 1;
            node = self.parents.get(node).unwrap();
        }
    }

    fn find_distance_to_san(&self) -> i32 {
        let mut cnt = 0;
        let mut node = self.parents.get("YOU").unwrap();
        while self.parents.contains_key(node) {
            if self.distance_to_san.contains_key(node) {
                return cnt + self.distance_to_san.get(node).unwrap();
            }
            cnt += 1;
            node = self.parents.get(node).unwrap();
        }
        panic!("failed to find");
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
            // dbg!(from);
            cnt += self.count_up(from);
        }
        cnt
    }
}

fn main() {
    let mut orbits = Orbits::parse("inputs/day6.txt");
    dbg!(orbits.count_orbits());
    orbits.mark_distances_to_san();
    // dbg!(orbits.distance_to_san);
    dbg!(orbits.find_distance_to_san());
}
