use std::io;
use std::io::prelude::*;

/// Kattis calls main function to run your solution.
fn main() {
    // Get standard input stream.
    let input = io::stdin();

    // Get input lines as strings.
    // See: https://doc.rust-lang.org/std/io/trait.BufRead.html
    let lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap())
        .collect::<Vec<String>>();


    let number_of_names: usize = lines[0].trim().parse().ok().unwrap();
    let mut fullnames: Vec<String> = Vec::new();
    for i in 1..=number_of_names{
        let firstname = &lines[i];
        let lastname = &lines[i+number_of_names];
        fullnames.push(format!("{} {}",firstname, lastname));
    }
    fullnames.sort();
    fullnames.dedup_by(|a,b| a==b);
    println!("{}", fullnames.len());
}
