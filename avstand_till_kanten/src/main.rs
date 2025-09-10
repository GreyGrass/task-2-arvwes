/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola Söderlund <violaso@kth.se>
 * Edit: Benjamin Widman <bwidman@kth.se>
 */

use std::cmp::min;
use std::io;
use std::io::prelude::*;

/// Kattis calls main function to run your solution.
fn main() {
    // get standard input stream
    let input = io::stdin();

    // Get input lines as a vector of strings
    // see: https://doc.rust-lang.org/std/io/trait.BufRead.html
    let mut lines = input
        .lines()
        .map(|_line| _line.ok().unwrap())
        .collect::<Vec<String>>(); // Converts iterator into vector. Not necessary, see example solution.
    // The map acts on every element in the iterator, getting the value inside the returned Result, assuming the result is Ok(value) and not Err(error_message).
    // ok() returns an Option, so we unwrap it to get the value inside.

    let numbers = lines[0]
        .split_whitespace()
        .filter_map(|string| string.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let mut rows = numbers[0];
    let mut columns = numbers[1];

    for i in 0..rows {
        for j in 0..columns {
            let mut distance_to_side = min(i + 1, rows - i);
            let mut distance_to_top = min(j + 1, columns - j);

            let distance_to_edge = min(distance_to_side, distance_to_top);

            if distance_to_edge >= 10 {
                print!(".");
            } else {
                print!("{}", distance_to_edge);
            }
        }
        println!()
    }
    eprintln!("Kattis skips this comment!");
    //println!("Print to standard output.");
}
/*
fn main() {
    println!("Hello, world!");
    let mut input1: String = String::new();
    let mut input2: String = String::new();

    io::stdin().read_line(&mut input1).expect("");

    io::stdin().read_line(&mut input2).expect("");

    let mut r: usize = input1.trim().parse().unwrap();
    let mut k: usize = input2.trim().parse().unwrap();

    println!();
    for i in 0..r {
        for j in 0..k {
            let mut distance_to_side = if i <= r / 2 {
                //print!("i{}", i+1);
                i + 1
                //  print!("ttest{}", distance_to_side);
            } else {
                //  print!("i{}", r-i);
                r - i
            };
            let mut distance_to_top = if j <= k / 2 {
                //print!("i{}", i+1);
                j + 1
            } else {
                //  print!("i{}", r-i);
                k - j
            };


            if distance_to_side <= distance_to_top {
                if distance_to_side < 10 {

                    print!("{}", distance_to_side);
                } else{
                           print!(".");
            }
        }else if distance_to_top < 10 {
                print!("{}", distance_to_top)
            } else{
                 print!(".");
            }
        }
        println!();
    }
}
    */
