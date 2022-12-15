use std::env;
use std::fs;

fn main() {
    let content = fs::read_to_string(env::args().nth(1).expect("expected a file"))
        .expect("could not load the file");

    let mut elves: Vec<usize> = content
        .trim()
        .split("\n\n")
        .map(|elf| {
            elf.to_string()
                .split("\n")
                .map(|x| x.to_string().parse::<usize>().unwrap())
                .sum()
        })
        .collect();

    println!("{}", elves.iter().max().unwrap());

    elves.sort();
    println!("{}", elves.iter().rev().take(3).sum::<usize>());
}
