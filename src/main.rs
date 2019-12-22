use std::env::args;
use std::fs;

#[derive(Debug)]
struct State {
    memory: Vec<usize>,
    /// Instruction pointer.
    ip: usize,
}

impl State {
    /// Creates State by parsing a comma delimited string of integers.
    fn from_string(s: &str) -> State {
        let v: Vec<usize> = s
            .trim()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        State { memory: v, ip: 0 }
    }

    /// Returns memory as comma-delimited string.
    fn memory_to_string(&self) -> String {
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
        println!("opcode: {}", opcode);
        let result = match opcode {
            1 | 2 => {
                println!("add");
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
    fn run(&mut self, debug: bool) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_to_string() {
        let state = State::from_string("1,2,3");
        assert_eq!(state.memory_to_string(), "1,2,3");
    }

    #[test]
    fn day2_example() {
        let mut state = State::from_string("1,9,10,3,2,3,11,0,99,30,40,50");
        state.run(false);
        assert_eq!(
            state.memory_to_string(),
            "3500,9,10,70,2,3,11,0,99,30,40,50"
        );
    }
}

fn main() {
    let filename = args().nth(1).expect("no filename given");
    let input = fs::read_to_string(filename).unwrap();
    let mut state = State::from_string(input.as_str());
    state.run(true);
}
