use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;

fn main() {
    let content = fs::read_to_string(env::args().nth(1).expect("expected a file"))
        .expect("could not load the file");

    let (piles, commands) = content.split_once("\n\n").unwrap();

    let mut stacks: HashMap<&str, VecDeque<char>> = piles
        .lines()
        .map(|line| {
            let (id, items) = line.split_once(" ").unwrap();
            (id, items.chars().collect::<VecDeque<char>>().clone())
        })
        .collect();

    let cmds = commands.to_string();

    for line in cmds.lines() {
        let cm: Vec<String> = line
            .split_whitespace()
            .map(|x| {
                x.to_string()
                    .chars()
                    .filter(|c| c.is_digit(10))
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .into_iter()
            .filter(|y| !y.is_empty())
            .collect();

        if let [mve, from, to] = &cm[..] {
            let mut mv = mve.parse::<u32>().unwrap();

            let f = stacks.get_mut(&from as &str);
            if let Some(f) = f {
                let mut to_push: Vec<char> = Vec::new();
                while mv > 0 {
                    to_push.push(f.pop_front().unwrap());
                    mv -= 1;
                }

                let t = stacks.get_mut(&to as &str);
                if let Some(t) = t {
                    for e in to_push.drain(..) {
                        t.push_front(e);
                    }
                }
            }
        }
    }

    let mut sorted_keys: Vec<&&str> = stacks.keys().collect();
    sorted_keys.sort();

    for e in sorted_keys {
        print!("{}", stacks.get(e).unwrap().get(0).unwrap());
    }
    println!("");
}
