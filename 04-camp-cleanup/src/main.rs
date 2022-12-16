use std::env;
use std::fs;

fn main() {
    let content = fs::read_to_string(env::args().nth(1).expect("expected a file"))
        .expect("could not load the file");

    let self_contained_ranges: u32 = content
        .lines()
        .map(|line| {
            let (lo, hi) = if let Some((lo, hi)) = line.split_once(",") { (lo, hi) } else { todo!() };
            let (a, b) = if let Some((a, b)) = lo.split_once("-") { (a, b) } else { todo!() };
            let (x, y) = if let Some((x, y)) = hi.split_once("-") { (x, y) } else { todo!() };

            let c = a.parse::<u32>().unwrap();
            let d = b.parse::<u32>().unwrap();

            let w = x.parse::<u32>().unwrap();
            let z = y.parse::<u32>().unwrap();

            ((c <= w && d >= z) || (w <= c && z >= d)) as u32
        })
        .sum();

    println!("{}", self_contained_ranges);

    // let overlaps: u32 = content
    //     .lines()
    //     .map(|line| {
    //         let (lo, hi) = if let Some((lo, hi)) = line.split_once(",") { (lo, hi) } else { todo!() };
    //         let (a, b) = if let Some((a, b)) = lo.split_once("-") { (a, b) } else { todo!() };
    //         let (x, y) = if let Some((x, y)) = hi.split_once("-") { (x, y) } else { todo!() };

    //         let c = a.parse::<u32>().unwrap();
    //         let d = b.parse::<u32>().unwrap();

    //         let w = x.parse::<u32>().unwrap();
    //         let z = y.parse::<u32>().unwrap();

    //         ((c <= w && d >= z) || (w <= c && z >= d)) as u32
    //     })
    //     .sum();

    // println!("{}", overlaps);
}
