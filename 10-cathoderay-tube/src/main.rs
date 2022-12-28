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
    last_cyle: usize,
}

impl Cpu {
    fn new(instructions: VecDeque<String>) -> Self {
        Self {
            instructions,
            x: 1,
            cycles: 0,
            to_add: HashMap::new(),
            last_cyle: 0,
        }
    }

    fn step(&mut self) {
        let instruction = self.instructions.pop_front().unwrap();
        let cmd: Vec<String> = instruction.split(" ").map(|cmd| cmd.to_string()).collect();

        if self.to_add.contains_key(&self.cycles) {
            self.x += self.to_add.get(&self.cycles).unwrap();
        }

        print!("Cycle: {}, ", self.cycles);
        match cmd.get(0).unwrap().as_str() {
            "addx" => {
                let index = self.cycles + 2;
                let value = cmd.get(1).unwrap().parse::<i32>().unwrap();

                // isn't there a max(val, val) where val derives ord?
                self.last_cyle = if index > self.last_cyle { index } else { self.last_cyle };

                println!("promise to add {} at cycle {}", value, index);
                self.to_add.insert(index, value)
            },
            "noop" => {
                println!("noop");
                Some(0)
            }
            _ => Some(0),
        };

        self.cycles += 1;
    }

    fn run(&mut self) -> i32 {
        while self.instructions.len() > 0 {
            self.step();
        }

        for cycle in self.cycles..self.last_cyle + 1 {
            print!("Cycle: {}, ", cycle);
            if self.to_add.contains_key(&cycle) {
                let value = self.to_add.get(&cycle).unwrap();
                println!("added {} to x", value);
                self.x += value;
            }
        }

        self.x
    }
}

fn main() {
    let content = fs::read_to_string(env::args().nth(1).expect("expected a file"))
        .expect("could not load the file");

    let instructions: VecDeque<String> = content.lines().map(|line| line.to_string()).collect();
    let mut cpu = Cpu::new(instructions);

    let xreg = cpu.run();
    println!("{}", xreg);
}
