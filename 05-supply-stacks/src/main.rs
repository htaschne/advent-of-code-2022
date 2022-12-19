use std::collections::HashMap;
use std::env;
use std::fs;

fn show(stacks: &HashMap<&str, Vec<char>>) {
    for (id, stack) in stacks {
        println!("{}: {:?}", id, stack);
    }

}

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
        show(&stacks);

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

        println!("move: {:?}", cm);

        if let [mve, from, to] = &cm[..] {
            let mut mv = mve.parse::<u32>().unwrap();

            let f = stacks.get_mut(&from as &str);
            if let Some(f) = f {
                let mut to_push: Vec<char> = Vec::new();
                while mv > 0 { 
                    to_push.push(f.remove(0));
                    mv -= 1;
                }

                println!("PUSHING: {:?}", to_push);
                let t = stacks.get_mut(&to as &str);
                if let Some(t) = t {
                    for e in to_push.drain(..) {
                        t.push(e);
                    }
                }

            }
        }
        // TODO: only print the last item in stack
        // show(&stacks);
        println!("----------")
    }


    // TODO: only print the last item in stack
    for (id, stack) in &stacks {
        println!("{}: {:?}", id, stack);
    }

}
