use std::env::args;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Moon {
    pos: [i32; 3],
    vel: [i32; 3],
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Sim {
    moons: Vec<Moon>,
}

impl Moon {
    fn parse(line: &str) -> Moon {
        // <x=-3, y=10, z=-1>
        // -3, y=10, z=-1
        let spl: Vec<&str> = line.trim().split(", ").collect();
        assert_eq!(spl.len(), 3);
        let x: i32 = spl[0].trim_start_matches("<x=").parse().unwrap();
        let y: i32 = spl[1].trim_start_matches("y=").parse().unwrap();
        let z: i32 = spl[2]
            .trim_start_matches("z=")
            .trim_end_matches('>')
            .parse()
            .unwrap();
        Moon {
            pos: [x, y, z],
            vel: [0, 0, 0],
        }
    }

    fn apply_acceleration(&mut self, vd: &[i32; 3]) {
        for i in 0..3 {
            self.vel[i] += vd[i];
        }
    }

    fn get_acceleration_from_gravity(&self, other: &Moon) -> [i32; 3] {
        let mut out = [0, 0, 0];
        for i in 0..3 {
            out[i] += match (self.pos[i], other.pos[i]) {
                (x, y) if x < y => 1,
                (x, y) if x > y => -1,
                _ => 0,
            };
        }
        out
    }

    fn apply_velocity(&mut self) {
        for i in 0..3 {
            self.pos[i] += self.vel[i];
        }
    }

    fn abs_sum(arr: &[i32]) -> i32 {
        let mut sum = 0;
        for el in arr.iter() {
            sum += el.abs();
        }
        sum
    }

    fn potential_energy(&self) -> i32 {
        Moon::abs_sum(&self.pos)
    }

    fn kinetic_energy(&self) -> i32 {
        Moon::abs_sum(&self.vel)
    }

    fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }

    fn zero_out_dimension(&mut self, i: usize) {
        self.pos[i] = 0;
        self.vel[i] = 0;
    }
}

impl Sim {
    fn parse(input: &str) -> Sim {
        let moons = input.lines().map(|line| Moon::parse(line)).collect();
        Sim { moons }
    }

    fn apply_gravity(&mut self) {
        for i in 0..self.moons.len() {
            for j in 0..self.moons.len() {
                if i == j {
                    continue;
                }
                let a = self.moons[i].get_acceleration_from_gravity(&self.moons[j]);
                self.moons[i].apply_acceleration(&a);
            }
        }
    }

    fn apply_velocities(&mut self) {
        for moon in self.moons.iter_mut() {
            moon.apply_velocity();
        }
    }

    fn step(&mut self) {
        self.apply_gravity();
        self.apply_velocities();
    }

    fn run(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step()
        }
    }

    fn total_energy(&self) -> i32 {
        let mut sum = 0;
        for moon in self.moons.iter() {
            sum += moon.total_energy();
        }
        sum
    }

    fn zero_out_dimension(&mut self, i: usize) {
        for moon in self.moons.iter_mut() {
            moon.zero_out_dimension(i);
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if a < b {
        gcd(b, a)
    } else if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(4, 6), 2);
        assert_eq!(gcd(6, 4), 2);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(6, 8), 24);
    }
}

fn find_period(start: &Sim) -> usize {
    let mut steps: [usize; 3] = [0, 0, 0];
    for dim in 0..3 {
        let mut sim = start.clone();
        for dim2 in 0..3 {
            if dim == dim2 {
                continue;
            }
            sim.zero_out_dimension(dim2);
        }
        let first = sim.clone();
        for i in 1..std::usize::MAX {
            sim.step();
            if i % 1_000_000 == 0 {
                dbg!(&i);
            }
            if sim == first {
                steps[dim] = i;
                dbg!(i);
                // dbg!(&sim);
                break;
            }
        }
    }
    lcm(steps[0], lcm(steps[1], steps[2]))
}

fn main() {
    let file = args().nth(1).expect("no filename given");
    let input = fs::read_to_string(file).unwrap();
    let first_state = Sim::parse(&input);
    dbg!(find_period(&first_state));
}
