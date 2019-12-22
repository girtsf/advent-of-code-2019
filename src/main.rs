use std::env::args;
use std::fs;

#[derive(Debug, Clone)]
pub struct State {
    memory: Vec<usize>,
    /// Instruction pointer.
    ip: usize,
}

impl State {
    /// Creates State by reading a file and parsing it as comma-delimited string of integers.
    pub fn from_file(path: &str) -> State {
        State::from_string(&fs::read_to_string(path).unwrap())
    }

    /// Creates State by parsing a comma delimited string of integers.
    pub fn from_string(s: &str) -> State {
        let v: Vec<usize> = s
            .trim()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        State { memory: v, ip: 0 }
    }

    /// Returns value at address 0.
    pub fn get_output(&self) -> usize {
        self.memory[0]
    }

    /// Returns memory as comma-delimited string.
    pub fn memory_to_string(&self) -> String {
        self.memory.iter().fold(
            String::new(),
            |s, x| if s.is_empty() { s } else { s + "," } + &x.to_string(),
        )
    }

    /// Reads value at given address, then reads value that address points at.
    fn read_indirect(&self, addr: usize) -> usize {
        self.memory[self.memory[addr]]
    }

    /// Writes value to address from given cell.
    fn write_indirect(&mut self, addr: usize, value: usize) {
        let write_addr = self.memory[addr];
        self.memory[write_addr] = value;
    }

    /// Steps the intcode computer by one step.
    ///
    /// Returns true iff we should keep executing.
    fn step(&mut self) -> bool {
        let opcode = self.memory[self.ip];
        // println!("opcode: {}", opcode);
        let result = match opcode {
            1 | 2 => {
                // println!("add");
                let in1 = self.read_indirect(self.ip + 1);
                let in2 = self.read_indirect(self.ip + 2);
                let write_addr = self.ip + 3;
                if opcode == 1 {
                    self.write_indirect(write_addr, in1 + in2);
                } else {
                    self.write_indirect(write_addr, in1 * in2);
                }
                true
            }
            99 => false,
            _ => {
                panic!("unknown opcode: {}", opcode);
            }
        };
        self.ip += 4;
        result
    }

    /// Runs until program finishes.
    pub fn run(&mut self, debug: bool) {
        loop {
            if debug {
                println!("{:?}", self);
            }
            if !self.step() {
                break;
            }
        }
        if debug {
            println!("{:?}", self);
        }
    }
}

/// "Find the input noun and verb that cause the program to produce the output 19690720. What is
/// 100 * noun + verb?"
pub fn day2_part2() {
    let mut state = State::from_file("inputs/day2_input.txt");
    for noun in 0..=99 {
        state.memory[1] = noun;
        for verb in 0..=99 {
            state.memory[2] = verb;
            let mut state2 = state.clone();
            state2.run(false);
            if state2.get_output() == 19690720 {
                println!(
                    "solution: noun={} verb={} combined={}",
                    noun,
                    verb,
                    100 * noun + verb
                );
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_to_string() {
        let state = State::from_string("1,2,3");
        assert_eq!(state.memory_to_string(), "1,2,3");
    }

    #[test]
    fn test_day2_example() {
        let mut state = State::from_string("1,9,10,3,2,3,11,0,99,30,40,50");
        state.run(false);
        assert_eq!(
            state.memory_to_string(),
            "3500,9,10,70,2,3,11,0,99,30,40,50"
        );
    }
}

fn main() {
    // day2_part2();
    let filename = args().nth(1).expect("no filename given");
    let mut state = State::from_file(&filename);
    state.run(true);
}
