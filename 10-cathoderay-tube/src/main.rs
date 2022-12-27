use std::collections::VecDeque;
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug)]
struct Cpu {
    instructions: VecDeque<String>,
    x: i32,
    cycles: usize,
    to_add: HashMap<usize, i32>,
}

impl Cpu {
    fn new(instructions: VecDeque<String>) -> Self {
        Self {
            instructions,
            x: 0,
            cycles: 0,
            to_add: HashMap::new(),
        }
    }

    fn step(&mut self) {
        let instruction = self.instructions.pop_front().unwrap();
        let cmd: Vec<String> = instruction.split(" ").map(|cmd| cmd.to_string()).collect();
        println!("{:?}", cmd);

        match cmd.get(0).unwrap().as_str() {
            "addx" => {
                let index = self.cycles + 2;
                let value = cmd.get(1).unwrap().parse::<i32>().unwrap();
                println!("{} {}", index, value);
                self.to_add.insert(index, value)
            },
            _ => Some(0),
        };

        self.cycles += 1;
        
    }

    fn run(&mut self) {
        while self.instructions.len() > 0 {
            self.step();
        }
    }
}

fn main() {
    let content = fs::read_to_string(env::args().nth(1).expect("expected a file"))
        .expect("could not load the file");

    let instructions: VecDeque<String> = content.lines().map(|line| line.to_string()).collect();
    let mut cpu = Cpu::new(instructions);

    cpu.run();
    println!("{}", cpu.cycles);
    println!("{}", cpu.x);
}
