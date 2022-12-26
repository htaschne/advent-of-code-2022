use std::collections::VecDeque;
use std::env;
use std::fs;

#[derive(Debug)]
struct Cpu {
    instructions: VecDeque<String>,
    x: i32,
    cycles: usize,
}

impl Cpu {
    fn new(instructions: VecDeque<String>) -> Self {
        Self {
            instructions,
            x: 0,
            cycles: 0,
        }
    }

    fn step(&mut self) {
        let ins = self.instructions.pop_front().unwrap();
        match ins {
            _ => panic!("could not match instruction"),
        }
        
    }
}

fn main() {
    let content = fs::read_to_string(env::args().nth(1).expect("expected a file"))
        .expect("could not load the file");

    let instructions: VecDeque<String> = content.lines().map(|line| line.to_string()).collect();
    let mut cpu = Cpu::new(instructions);

    cpu.step();
    println!("HELLO");
}
