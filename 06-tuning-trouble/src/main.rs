use std::collections::HashSet;
use std::env;
use std::fs;

fn find_marker(buffer: &Vec<char>, chunk_size: usize) -> usize {
    for (index, element) in buffer
        .windows(chunk_size)
        .map(|c| c.into_iter().collect::<HashSet<&char>>().len() == chunk_size)
        .enumerate()
    {
        if element {
            return index + chunk_size;
        }
    }
    return 0;
}

fn main() {
    let content: Vec<char> = fs::read_to_string(env::args().nth(1).expect("expected a file"))
        .expect("could not load the file")
        .trim()
        .chars()
        .collect();

    let first_marker = find_marker(&content, 4);
    println!("{}", first_marker);

    let first_marker = find_marker(&content, 14);
    println!("{}", first_marker);
}
