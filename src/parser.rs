//! This module provide all the functions you could ever need for opening files or reading
//! arguments
//!
//! Since all the input files are quite small in the advent of code we are putting the full file
//! into RAM instead of using a BufReader.

/// Provide an iterator over the chars of a file.
/// ```no_run
/// for line in aoc::parser::chars_from_file("input") {
///     // things
/// }
/// ```
pub fn chars_from_file(filename: &str) -> impl Iterator<Item = char> {
    let file = Box::new(read_file(filename));
    let file = Box::leak(file);
    file.chars()
}

/// Provide an iterator over the chars of the file specified by the position of an argument
/// ```no_run
/// for c in aoc::parser::chars_from_args(1) {
///     // things
/// }
/// ```
/// Usually you'll want to use it with `1` because `0` is the name of your own program
pub fn chars_from_args(n: usize) -> impl Iterator<Item = char> {
    let filename = get_args(n).expect("give me the path to your program");

    chars_from_file(&filename)
}
/// Provide an iterator over the lines of a file
/// ```no_run
/// for line in aoc::parser::lines_from_file("input") {
///     // things
/// }
/// ```
pub fn lines_from_file(filename: &str) -> impl Iterator<Item = String> {
    let file = Box::new(read_file(filename));
    let file = Box::leak(file);
    file.lines().map(|line| line.to_owned())
}

/// Provide an iterator over the lines of the file specified by the position of an argument
/// ```no_run
/// for line in aoc::parser::lines_from_args(1) {
///     // things
/// }
/// ```
/// Usually you'll want to use it with `1` because `0` is the name of your own program
pub fn lines_from_args(n: usize) -> impl Iterator<Item = String> {
    let filename = get_args(n).expect("give me the path to your program");

    lines_from_file(&filename)
}

/// Read a whole file into a string
/// ```no_run
/// let input = aoc::parser::read_file("input");
/// ```
pub fn read_file(filename: &str) -> String {
    std::str::from_utf8(&std::fs::read(filename).unwrap())
        .expect("I was unable to parse your file to valid UTF-8")
        .into()
}

/// Read a whole file into a string from the position of an argument
/// ```no_run
/// let input = aoc::parser::lines_from_args(1);
/// ```
/// Usually you'll want to use it with `1` because `0` is the name of your own program
pub fn read_file_from_args(n: usize) -> String {
    std::str::from_utf8(&std::fs::read(get_args(n).expect("Give the path of your file")).unwrap())
        .expect("I was unable to parse your file to valid UTF-8")
        .into()
}

/// Provie the argument at the position `n`:
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
