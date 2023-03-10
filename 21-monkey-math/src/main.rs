use std::collections::HashMap;
use std::env;
use std::fs;
use std::cmp;

fn find(root: &str, graph: &HashMap<&str, &str>) -> i64 {
    match graph.get(root).unwrap().parse::<i64>() {
        Ok(value) => value,
        Err(_) => {
            let operation = graph
                .get(root)
                .unwrap()
                .split(" ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            match operation.get(1).unwrap().to_string().as_str() {
                "+" => {
                    find(operation.get(0).unwrap(), graph) + find(operation.get(2).unwrap(), graph)
                }
                "-" => {
                    find(operation.get(0).unwrap(), graph) - find(operation.get(2).unwrap(), graph)
                }
                "*" => {
                    find(operation.get(0).unwrap(), graph) * find(operation.get(2).unwrap(), graph)
                }
                "/" => {
                    find(operation.get(0).unwrap(), graph) / find(operation.get(2).unwrap(), graph)
                }
                "=" => {
                    let lo = find(operation.get(0).unwrap(), graph);
                    let hi = find(operation.get(2).unwrap(), graph);

                    let mx = cmp::max(lo, hi);
                    let mn = cmp::min(lo, hi);
                    
                    let val = mx - mn;

                    println!("{} {}: {}", mn, mx, val);
                    // println!("{}", val + mx + 1);
                    val + mx + 1
                }
                _ => panic!("could not mactch operation {}", operation.get(1).unwrap()),
            }
        }
    }
}

fn main() {
    let content = fs::read_to_string(env::args().nth(1).expect("expected a file"))
        .expect("could not load the file");

    let graph = content
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .collect::<HashMap<&str, &str>>();

    let target = "root";
    let result = find(target, &graph);
    println!("{}", result);
}
