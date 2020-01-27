use std::env::args;
use std::fs;

#[derive(Debug)]
struct Moon {
    pos: [i32; 3],
    vel: [i32; 3],
}

#[derive(Debug)]
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
}

fn main() {
    let file = args().nth(1).expect("no filename given");
    let input = fs::read_to_string(file).unwrap();
    let mut sim = Sim::parse(&input);
    sim.run(1000);
    dbg!(&sim);
    dbg!(&sim.total_energy());
}
