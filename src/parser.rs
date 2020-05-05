use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn lines_from_file(filename: &str) -> impl Iterator<Item = String> {
    let file = File::open(filename).expect("Can’t open file");
    let reader = BufReader::new(file);

    reader.lines().filter_map(|line| line.ok())
}

pub fn lines_from_args(n: usize) -> impl Iterator<Item = String> {
    let filename = std::env::args()
        .skip(n)
        .next()
        .expect("give me the path to your program");

    lines_from_file(&filename)
}
