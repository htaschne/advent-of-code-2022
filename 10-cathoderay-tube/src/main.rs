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
    strength_signals: Vec<i32>,
}

impl Cpu {
    fn new(instructions: VecDeque<String>) -> Self {
        Self {
            instructions,
            x: 1,
            cycles: 0,
            to_add: HashMap::new(),
            last_cyle: 0,
            strength_signals: Vec::new(),
        }
    }

    fn step(&mut self) {
        let instruction = self.instructions.pop_front().unwrap();
        let cmd: Vec<String> = instruction.split(" ").map(|cmd| cmd.to_string()).collect();

        // print!("Cycle: {}, ", self.cycles);

        if self.cycles % 20 == 0 && self.cycles != 0 {
            println!("[{}]: {}", self.cycles, self.x * self.cycles as i32);
            self.strength_signals.push(self.x * self.cycles as i32);
        }

        match cmd.get(0).unwrap().as_str() {
            "addx" => {
                let index = self.cycles + 2;
                self.last_cyle = if index > self.last_cyle { index } else { self.last_cyle };
                let value = cmd.get(1).unwrap().parse::<i32>().unwrap();

                // println!("promise to add {} at cycle {}", value, index);
                self.to_add.insert(index, value);
            },
            "noop" => {
                // println!("noop");
                ()
            }
            _ => (),
        };

        if self.to_add.contains_key(&self.cycles) {
            self.x += self.to_add.get(&self.cycles).unwrap().clone();
        }

        self.cycles += 1;
    }

    fn run(&mut self) -> i32 {
        while self.instructions.len() > 0 {
            self.step();
        }

        for cycle in self.cycles..self.last_cyle + 1 {
            // print!("Cycle: {}, ", cycle);

            if cycle % 20 == 0 {
                println!("[{}]: {}", cycle, self.x * cycle as i32);
                self.strength_signals.push(self.x * cycle as i32);
            }

            if self.to_add.contains_key(&cycle) {
                let value = self.to_add.get(&cycle).unwrap();
                // println!("added {} to x", value);
                self.x += value;
            }
        }

        self.strength_signals.iter().sum()
    }
}

fn main() {
    let content = fs::read_to_string(env::args().nth(1).expect("expected a file"))
        .expect("could not load the file");

    let instructions: VecDeque<String> = content.lines().map(|line| line.to_string()).collect();
    let mut cpu = Cpu::new(instructions);

    let strength_signal = cpu.run();
    println!("{}", strength_signal);
}
