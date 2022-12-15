use std::env;
use std::fs;

fn main() {
    let content = fs::read_to_string(env::args().nth(1).expect("expected a file"))
        .expect("could not load the file");

    let highest_calorie: usize = content
        .trim()
        .split("\n\n")
        .map(|elf| {
            elf.to_string()
                .split("\n")
                .map(|x| x.to_string().parse::<usize>().unwrap())
                .sum()
        })
        .max()
        .unwrap();

    println!("{}", highest_calorie);
}
