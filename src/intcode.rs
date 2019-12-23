use std::collections::{HashMap, VecDeque};

/// Intcode interpreter.
use std::fs;

#[derive(Debug, Default, Clone)]
pub struct State {
    memory: HashMap<i64, i64>,
    /// Instruction pointer.
    ip: i64,

    inputs: VecDeque<i64>,
    outputs: Vec<i64>,
    relative_base: i64,
}

/// Parses comma-delimited string into a vector of ints.
fn parse_ints(s: &str) -> Vec<i64> {
    s.trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn parse_ints_to_hashmap(s: &str) -> HashMap<i64, i64> {
    s.trim()
        .split(',')
        .enumerate()
        .map(|(i, x)| (i as i64, x.parse::<i64>().unwrap()))
        .collect()
}

impl State {
    /// Creates State by reading a file and parsing it as comma-delimited string of integers.
    pub fn from_file(path: &str) -> State {
        State::from_string(&fs::read_to_string(path).unwrap())
    }

    /// Creates State by parsing a comma delimited string of integers.
    pub fn from_string(s: &str) -> State {
        State {
            memory: parse_ints_to_hashmap(s),
            ..Default::default()
        }
    }

    /// Returns memory as comma-delimited string.
    ///
    /// If there are no holes, memory is returned as "123,456,..etc..", where 123 is the value at
    /// address 0, 456 is the value at address 1, etc.
    ///
    /// If there are holes, every time a hole is enountered, we add @add, e.g., "123,@10=456".
    fn memory_to_string(&self) -> String {
        let mut addrs: Vec<i64> = self.memory.keys().cloned().collect();
        addrs.sort();
        let mut prev_addr: i64 = -1;

        addrs.iter().fold(String::new(), |s, addr| {
            let mut added: String = if s.is_empty() {
                String::from("")
            } else {
                String::from(",")
            };
            if *addr != (prev_addr + 1) {
                added += "@";
                added += &addr.to_string();
                added += "=";
            };
            let value = self.memory[addr];
            added += &value.to_string();
            prev_addr = *addr;
            String::from(s) + &added
        })
    }

    /// Reads value at given address.
    fn read(&self, addr: i64) -> i64 {
        assert!(addr >= 0);
        *self.memory.get(&addr).unwrap_or(&0)
    }

    /// Reads value at given address, then reads value that address points at.
    fn read_indirect(&self, addr: i64) -> i64 {
        let read_addr = self.read(addr);
        self.read(read_addr)
    }

    /// Writes value to given address.
    fn write(&mut self, addr: i64, value: i64) {
        assert!(addr >= 0);
        self.memory.insert(addr, value);
    }

    /// Reads immediate or position, based on mode.
    fn read_with_mode(&self, mode: i64, value: i64) -> i64 {
        match mode {
            0 => self.read(value),
            1 => value,
            2 => self.read(self.relative_base + value),
            _ => panic!("invalid mode: {}", mode),
        }
    }

    /// Writes immediate or position, based on mode.
    fn write_with_mode(&mut self, mode: i64, addr: i64, value: i64) {
        match mode {
            0 => self.write(addr, value),
            1 => panic!("write with immediate mode parameter"),
            2 => self.write(self.relative_base + addr, value),
            _ => panic!("invalid mode: {}", mode),
        }
    }

    /// Returns first argument after instruction pointer.
    fn arg1(&self) -> i64 {
        self.read(self.ip + 1)
    }

    /// Returns first argument after instruction pointer.
    fn arg2(&self) -> i64 {
        self.read(self.ip + 2)
    }

    /// Returns first argument after instruction pointer.
    fn arg3(&self) -> i64 {
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
                let result = if instruction == 01 {
                    lhs + rhs
                } else {
                    lhs * rhs
                };
                self.write_with_mode(mode3, self.arg3(), result);
                self.ip += 4;
            }
            // Input:
            03 => {
                if self.inputs.is_empty() {
                    panic!("tried to read from input, but input was empty :(");
                }
                let value = self.inputs.pop_front().unwrap();
                // println!("INPUT: writing {} to {}", value, write_addr);
                self.write_with_mode(mode1, self.arg1(), value);
                self.ip += 2;
            }
            // Output:
            04 => {
                let value = self.read_with_mode(mode1, self.arg1());
                println!("OUTPUT: {}", value);
                self.outputs.push(value);
                self.ip += 2;
            }
            // 05: Jump-if-true:
            // 06: Jump-if-false:
            05 | 06 => {
                let p1 = self.read_with_mode(mode1, self.arg1());
                let p2 = self.read_with_mode(mode2, self.arg2());
                if ((instruction == 05) && (p1 != 0)) || ((instruction == 06) && (p1 == 0)) {
                    self.ip = p2;
                } else {
                    self.ip += 3;
                }
            }
            // 07: less than:
            // 08: equals:
            07 | 08 => {
                let p1 = self.read_with_mode(mode1, self.arg1());
                let p2 = self.read_with_mode(mode2, self.arg2());
                let value =
                    ((instruction == 07) && (p1 < p2)) || ((instruction == 08) && (p1 == p2));
                self.write_with_mode(mode3, self.arg3(), if value { 1 } else { 0 });
                self.ip += 4;
            }
            09 => {
                self.relative_base += self.read_with_mode(mode1, self.arg1());
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

    pub fn add_input(&mut self, val: i64) {
        self.inputs.push_back(val);
    }

    pub fn outputs(&self) -> &Vec<i64> {
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
pub fn day2_part2() -> i64 {
    let mut state = State::from_file("inputs/day2_input.txt");
    for noun in 0..=99 {
        state.write(1, noun);
        for verb in 0..=99 {
            state.write(2, verb);
            let mut state2 = state.clone();
            state2.run(false);
            if state2.read(0) == 19690720 {
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
    fn test_day5_example() {
        let mut state = State::from_string("1002,4,3,4,33");
        state.run(true);
        assert_eq!(state.memory_to_string(), "1002,4,3,4,99");
    }

    #[test]
    fn test_day5_input() {
        let mut state = State::from_string("3,0,99");
        state.add_input(-42);
        state.run(true);
        assert_eq!(state.memory_to_string(), "-42,0,99");
    }

    #[test]
    fn test_day5_output() {
        let mut state = State::from_string("104,42,4,0,99");
        state.run(true);
        assert_eq!(state.memory_to_string(), "104,42,4,0,99");
        assert_eq!(state.outputs, vec![42, 104]);
    }

    const JUMP_POSITION_TEST: &str = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";

    #[test]
    fn test_day5_jump_test1() {
        let mut state = State::from_string(JUMP_POSITION_TEST);
        state.add_input(123);
        state.run(true);
        assert_eq!(state.outputs, vec![1]);
    }

    #[test]
    fn test_day5_jump_test2() {
        let mut state = State::from_string(JUMP_POSITION_TEST);
        state.add_input(0);
        state.run(true);
        assert_eq!(state.outputs, vec![0]);
    }

    const JUMP_IMMEDIATE_TEST: &str = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";

    #[test]
    fn test_day5_jump_test3() {
        let mut state = State::from_string(JUMP_IMMEDIATE_TEST);
        state.add_input(123);
        state.run(true);
        assert_eq!(state.outputs, vec![1]);
    }

    #[test]
    fn test_day5_jump_test4() {
        let mut state = State::from_string(JUMP_IMMEDIATE_TEST);
        state.add_input(0);
        state.run(true);
        assert_eq!(state.outputs, vec![0]);
    }

    #[test]
    fn test_day5_compare() {
        // List of tests to run. Each element consists of program code, and an array of input and
        // output value test cases.
        const TESTS: &[(&str, &[(i64, i64)])] = &[
            ("3,9,8,9,10,9,4,9,99,-1,8", &[(9, 0), (8, 1), (7, 0)]),
            ("3,9,7,9,10,9,4,9,99,-1,8", &[(9, 0), (8, 0), (7, 1)]),
            ("3,3,1108,-1,8,3,4,3,99", &[(9, 0), (8, 1), (7, 0)]),
            ("3,3,1107,-1,8,3,4,3,99", &[(9, 0), (8, 0), (7, 1)]),
        ];

        for (code, test_cases) in TESTS.iter() {
            for (input, expected_output) in test_cases.iter() {
                let mut state = State::from_string(code);
                state.add_input(*input);
                state.run(true);
                assert_eq!(state.outputs, vec![*expected_output]);
            }
        }
    }

    #[test]
    fn test_day5_larger_example() {
        const TEST_CASES: &[(i64, i64)] = &[(5, 999), (8, 1000), (1234, 1001)];
        for (input, expected_output) in TEST_CASES.iter() {
            let mut state = State::from_string(concat!(
                "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0",
                ",1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20",
                ",1105,1,46,98,99"
            ));
            state.add_input(*input);
            state.run(true);
            assert_eq!(state.outputs, vec![*expected_output]);
        }
    }

    #[test]
    fn test_day9_quine() {
        let code = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut state = State::from_string(code);
        state.run(false);
        assert_eq!(state.outputs, parse_ints(code));
    }

    #[test]
    fn test_day9_long_number() {
        let code = "1102,34915192,34915192,7,4,7,99,0";
        let mut state = State::from_string(code);
        state.run(false);
        assert_eq!(state.outputs, vec![1219070632396864]);
    }

    #[test]
    fn test_day9_long_number2() {
        let code = "104,1125899906842624,99";
        let mut state = State::from_string(code);
        state.run(false);
        assert_eq!(state.outputs, vec![1125899906842624]);
    }
}
