use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let content = fs::read_to_string(env::args().nth(1).expect("expected a file"))
        .expect("could not load the file");

    let (piles, commands) = content.split_once("\n\n").unwrap();

    let mut stacks: HashMap<&str, Vec<char>> = piles
        .lines()
        .map(|line| {
            let (id, items) = line.split_once(" ").unwrap();
            (id, items.chars().collect::<Vec<char>>().clone())
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

        println!("{:?}", cm);

        if let [mve, from, to] = &cm[..] {
            let mv = mve.parse::<u32>().unwrap();

            let f = stacks.get_mut(&from as &str);
            if let Some(f) = f {
                let mut to_push: Vec<char> = Vec::new();
                for _ in 1..mv {
                    to_push.push(f.pop().unwrap());
                }

                let t = stacks.get_mut(&to as &str);
                if let Some(t) = t {
                    for e in to_push {
                        t.push(e);
                    }
                }
            }
        }
    }


    // TODO: only print the last item in stack
    // for (id, stack) in &stacks {
    //     println!("{}: {:?}", id, stack);
    // }

}
