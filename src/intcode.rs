use std::collections::VecDeque;
/// Intcode interpreter.
use std::fs;

#[derive(Debug, Default, Clone)]
pub struct State {
    memory: Vec<i32>,
    /// Instruction pointer.
    ip: i32,

    inputs: VecDeque<i32>,
    outputs: Vec<i32>,
}

impl State {
    /// Creates State by reading a file and parsing it as comma-delimited string of integers.
    pub fn from_file(path: &str) -> State {
        State::from_string(&fs::read_to_string(path).unwrap())
    }

    /// Creates State by parsing a comma delimited string of integers.
    pub fn from_string(s: &str) -> State {
        let v: Vec<i32> = s
            .trim()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        State {
            memory: v,
            ..Default::default()
        }
    }

    /// Returns memory as comma-delimited string.
    fn memory_to_string(&self) -> String {
        self.memory.iter().fold(
            String::new(),
            |s, x| if s.is_empty() { s } else { s + "," } + &x.to_string(),
        )
    }

    fn num_to_addr_usize(&self, addr: i32) -> usize {
        if addr < 0 {
            panic!("address {} is below zero", addr);
        } else if addr >= self.memory.len() as i32 {
            panic!(
                "address {} is higher than memory size {}",
                addr,
                self.memory.len()
            );
        }
        addr as usize
    }

    /// Reads value at given address.
    fn read(&self, addr: i32) -> i32 {
        self.memory[self.num_to_addr_usize(addr)]
    }

    /// Reads value at given address, then reads value that address points at.
    fn read_indirect(&self, addr: i32) -> i32 {
        let read_addr = self.read(addr);
        self.memory[self.num_to_addr_usize(read_addr)]
    }

    /// Writes value to given address.
    fn write(&mut self, addr: i32, value: i32) {
        let write_addr_usize = self.num_to_addr_usize(addr);
        self.memory[write_addr_usize] = value;
    }

    /// Writes value to address from given cell.
    fn write_indirect(&mut self, addr: i32, value: i32) {
        let write_addr = self.read(addr);
        let write_addr_usize = self.num_to_addr_usize(write_addr);
        self.memory[write_addr_usize] = value;
    }

    /// Reads immediate or position, based on mode.
    fn read_with_mode(&self, mode: i32, value: i32) -> i32 {
        match mode {
            0 => self.read(value),
            1 => value,
            _ => panic!("invalid mode: {}", mode),
        }
    }

    /// Returns first argument after instruction pointer.
    fn arg1(&self) -> i32 {
        self.read(self.ip + 1)
    }

    /// Returns first argument after instruction pointer.
    fn arg2(&self) -> i32 {
        self.read(self.ip + 2)
    }

    /// Returns first argument after instruction pointer.
    fn arg3(&self) -> i32 {
        self.read(self.ip + 3)
    }

    /// Steps the intcode computer by one step.
    ///
    /// Returns true iff we should keep executing.
    fn step(&mut self) -> bool {
        let opcode = self.read(self.ip);
        assert!(opcode >= 0);

        let instruction = opcode % 100;
        let mode1 = (opcode / 100) % 10;
        let mode2 = (opcode / 1000) % 10;
        let mode3 = (opcode / 10000) % 10;
        match instruction {
            // Add or multiply:
            01 | 02 => {
                let lhs = self.read_with_mode(mode1, self.arg1());
                let rhs = self.read_with_mode(mode2, self.arg2());
                // Writes are always in position mode.
                assert!(mode3 == 0);
                let write_addr = self.arg3();
                let result = if instruction == 01 {
                    lhs + rhs
                } else {
                    lhs * rhs
                };
                self.write(write_addr, result);
                self.ip += 4;
            }
            // Input:
            03 => {
                // Writes are always in position mode.
                assert!(mode1 == 0);
                assert!(!self.inputs.is_empty());
                let write_addr = self.arg1();
                let value = self.inputs.pop_front().unwrap();
                println!("INPUT: writing {} to {}", value, write_addr);
                self.write(write_addr, value);
                self.ip += 2;
            }
            // Output:
            04 => {
                let value = self.read_with_mode(mode1, self.arg1());
                println!("OUTPUT: {}", value);
                self.outputs.push(value);
                self.ip += 2;
            }
            99 => {
                self.ip += 1;
                if !self.inputs.is_empty() {
                    panic!("not all inputs were used!");
                }
                return false;
            }
            _ => {
                panic!("unknown instruction/opcode: {}", opcode);
            }
        };
        true
    }

    pub fn add_input(&mut self, val: i32) {
        self.inputs.push_back(val);
    }

    pub fn outputs(&self) -> &Vec<i32> {
        &self.outputs
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
pub fn day2_part2() -> i32 {
    let mut state = State::from_file("inputs/day2_input.txt");
    for noun in 0..=99 {
        state.memory[1] = noun;
        for verb in 0..=99 {
            state.memory[2] = verb;
            let mut state2 = state.clone();
            state2.run(false);
            if state2.memory[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("could not find answer");
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

    #[test]
    fn test_day2_part2() {
        assert_eq!(day2_part2(), 9425);
    }

    #[test]
    fn test_day3_example() {
        let mut state = State::from_string("1002,4,3,4,33");
        state.run(true);
        assert_eq!(state.memory_to_string(), "1002,4,3,4,99");
    }

    #[test]
    fn test_day3_input() {
        let mut state = State::from_string("3,0,99");
        state.add_input(-42);
        state.run(true);
        assert_eq!(state.memory_to_string(), "-42,0,99");
    }

    #[test]
    fn test_day3_output() {
        let mut state = State::from_string("104,42,4,0,99");
        state.run(true);
        assert_eq!(state.memory_to_string(), "104,42,4,0,99");
        assert_eq!(state.outputs, vec![42, 104]);
    }
}
