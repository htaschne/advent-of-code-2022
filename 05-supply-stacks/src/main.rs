use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;

fn run(
    stacks: &HashMap<&str, VecDeque<char>>,
    commands: &Vec<Vec<String>>,
    crate_mover_9001: bool,
) -> String {
    let mut stk = stacks.clone();

    for cmd in commands {
        if let [mve, from, to] = &cmd[..] {
            let mut mv = mve.parse::<u32>().unwrap();

            let f = stk.get_mut(&from as &str);
            if let Some(f) = f {
                let mut to_push: Vec<char> = Vec::new();
                while mv > 0 {
                    to_push.push(f.pop_front().unwrap());
                    mv -= 1;
                }

                let t = stk.get_mut(&to as &str);
                if let Some(t) = t {
                    if crate_mover_9001 {
                        for e in to_push.drain(..).rev() {
                            t.push_front(e);
                        }
                    } else {
                        for e in to_push.drain(..) {
                            t.push_front(e);
                        }
                    }
                }
            }
        }
    }

    let mut sorted_keys: Vec<&&str> = stk.keys().collect();
    sorted_keys.sort();

    sorted_keys
        .iter()
        .map(|e| stk.get(&e as &str).unwrap().get(0).unwrap())
        .collect::<String>()
}

fn main() {
    let content = fs::read_to_string(env::args().nth(1).expect("expected a file"))
        .expect("could not load the file");

    let (piles, commands) = content.split_once("\n\n").unwrap();

    let stacks: HashMap<&str, VecDeque<char>> = piles
        .lines()
        .map(|line| {
            let (id, items) = line.split_once(" ").unwrap();
            (id, items.chars().collect::<VecDeque<char>>().clone())
        })
        .collect();

    let cmds: Vec<Vec<String>> = commands
        .to_string()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| {
                    x.to_string()
                        .chars()
                        .filter(|c| c.is_digit(10))
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .into_iter()
                .filter(|y| !y.is_empty())
                .collect::<Vec<String>>()
        })
        .collect();

    println!("{}", run(&stacks, &cmds, false));
    println!("{}", run(&stacks, &cmds, true));
}
