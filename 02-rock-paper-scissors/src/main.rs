use std::env;
use std::fs;

fn main() {
    let content = fs::read_to_string(env::args().nth(1).expect("expected a file"))
        .expect("could not load the file");

    let mut score: usize = 0;
    for line in content.trim().split("\n") {
        let (cpu, me) = line.split_once(" ").unwrap();

        score += match me {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!("could not match {}", me),
        };

        match (cpu, me) {
            ("C", "X") | ("A", "Y") | ("B", "Z") => score += 6,
            ("A", "X") | ("B", "Y") | ("C", "Z") => score += 3,
            _ => (),
        }
    }

    println!("{}", score);
}
