use std::collections::HashSet;
use std::env;
use std::fs;

fn sum_all(intersections: Vec<char>) -> u32 {
    intersections
        .iter()
        .map(|x| match x {
            'a'..='z' => *x as u32 - 'a' as u32 + 1,
            'A'..='Z' => *x as u32 - 38,
            _ => panic!("could not match {}", x),
        })
        .sum()
}

fn main() {
    let content = fs::read_to_string(env::args().nth(1).expect("expected a file"))
        .expect("could not load the file");

    let intersections: Vec<char> = content
        .lines()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            a.chars()
                .collect::<HashSet<char>>()
                .intersection(&(b.chars().collect::<HashSet<char>>()))
                .next()
                .unwrap()
                .clone()
        })
        .collect();

    println!("{}", sum_all(intersections));

    let intersections: Vec<char> = content
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|x| {
            let a = x[0].chars().collect::<HashSet<char>>();
            let b = x[1].chars().collect::<HashSet<char>>();
            let c = x[2].chars().collect::<HashSet<char>>();

            let it: HashSet<char> = a.intersection(&b).cloned().collect();
            it.intersection(&c)
                .cloned()
                .collect::<HashSet<char>>()
                .iter()
                .next()
                .unwrap()
                .clone()
        })
        .collect();

    println!("{}", sum_all(intersections));
}
