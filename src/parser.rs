//! Provide all the functions you could ever need for opening files or reading
//! arguments.
//!
//! Since all the input files are quite small in the advent of code we are putting the full file
//! into RAM instead of using a BufReader.

use std::{
    io::{stdin, Read},
    str::FromStr,
};

/// Read a whole file into a string
/// ```no_run
/// let input = aoc::parser::read_file("input");
/// ```
fn read_file(filename: &str) -> String {
    std::str::from_utf8(&std::fs::read(filename).unwrap())
        .expect("I was unable to parse your file to valid UTF-8")
        .into()
}

/// Read a whole file into a string from `stdin`
/// ```no_run
/// let input = aoc::parser::read_file_from_stdin();
/// ```
fn read_file_from_stdin() -> String {
    let mut buffer = Vec::new();
    stdin().read_to_end(&mut buffer).unwrap();
    std::str::from_utf8(&buffer).unwrap().to_string()
}

/// Provide the argument at the position `n`:
/// ```no_run
/// let exe = aoc::parser::get_args(0).unwrap(); // the name of the executable
/// let input = match aoc::parser::get_args(1) { // the first argument after the executable
///     Some(i) => i,
///     None => panic!(), // do something
/// };
/// ```
pub fn get_args(n: usize) -> Option<String> {
    std::env::args().nth(n)
}

/// Provide the input in a `String`.
/// Will look for your input:
/// 1. In `stdin`.
/// 2. In the filename specified in `args[1]`.
/// 3. In a file named `input` in the current directory.
/// ```no_run
/// let input: String = aoc::parser::input();
/// ```
pub fn input<T: FromStr>() -> T {
    if atty::isnt(atty::Stream::Stdin) {
        read_file_from_stdin()
    } else if let Some(filename) = get_args(1) {
        read_file(&filename)
    } else {
        std::str::from_utf8(&std::fs::read("input").expect("You need to provide an input. You can either pipe your input in `cargo run`, provide your file name in to `cargo run` or name your input file `input`")).unwrap().to_string()
    }.parse().ok()
    .expect("Could not parse the input in the expected type")
}

/// Provide an [`Iterator`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html)
/// over the input's line parsed into any type implementing
/// [`FromStr`](https://doc.rust-lang.org/stable/std/str/trait.FromStr.html).
/// ```no_run
/// let input: Vec<String> = aoc::parser::lines().collect();
/// ```
pub fn lines<T: FromStr>() -> impl Iterator<Item = T> {
    let input = Box::new(input::<String>());
    let input = Box::leak(input);
    input.lines().map(|line| {
        line.to_owned()
            .parse()
            .ok()
            .unwrap_or_else(|| panic!("Could not parse the following line: {}", line))
    })
}

/// Provide an [`Iterator`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html)
/// over the input's `char` parsed into any type implementing
/// [`FromStr`](https://doc.rust-lang.org/stable/std/str/trait.FromStr.html).
/// ```no_run
/// let input: Vec<u8> = aoc::parser::chars().collect();
/// ```
pub fn chars<T: FromStr>() -> impl Iterator<Item = T> {
    let input = Box::new(input::<String>());
    let input = Box::leak(input);
    input.chars().map(|c| {
        c.to_string()
            .parse()
            .ok()
            .unwrap_or_else(|| panic!("Could not parse the following char: {}", c))
    })
}
