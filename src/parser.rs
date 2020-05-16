use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn lines_from_file(filename: &str) -> impl Iterator<Item = String> {
    let file = File::open(filename).expect("Can’t open file");
    let reader = BufReader::new(file);

    reader.lines().filter_map(|line| line.ok())
}

pub fn lines_from_args(n: usize) -> impl Iterator<Item = String> {
    let filename = get_args(n).expect("give me the path to your program");

    lines_from_file(&filename)
}

pub fn read_file_from_args(n: usize) -> String {
    std::str::from_utf8(&std::fs::read(get_args(n).expect("Give the path of your file")).unwrap())
        .expect("I was unable to parse your file to valid UTF-8")
        .into()
}

pub fn get_args(n: usize) -> Option<String> {
    std::env::args().skip(n).next()
}
