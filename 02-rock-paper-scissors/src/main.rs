use std::env;
use std::fs;

fn wrong_strategy(x: (&str, &str)) -> usize {
    let mut score = 0;
    let (cpu, me) = x;

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

    score
}

fn right_strategy(x: (&str, &str)) -> usize {
    let mut score = 0;
    let (cpu, game_result) = x;

    match (cpu, game_result) {
        ("A", "X") => score += 3,
        ("B", "X") => score += 1,
        ("C", "X") => score += 2,

        ("A", "Y") => score += 1 + 3,
        ("B", "Y") => score += 2 + 3,
        ("C", "Y") => score += 3 + 3,

        ("A", "Z") => score += 2 + 6,
        ("B", "Z") => score += 3 + 6,
        ("C", "Z") => score += 1 + 6,

        _ => panic!("could not match {:?}", x),
    }

    score
}

fn play(content: &String, strategy: &dyn Fn((&str, &str)) -> usize) -> usize {
    content
        .trim()
        .split("\n")
        .map(|line| strategy(line.split_once(" ").unwrap()))
        .sum()
}

fn main() {
    let content = fs::read_to_string(env::args().nth(1).expect("expected a file"))
        .expect("could not load the file");

    let wscore = play(&content, &wrong_strategy);
    let rscore = play(&content, &right_strategy);

    println!("{} {}", wscore, rscore);
}
