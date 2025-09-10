
use std::io;
fn main() {
      // get standard input stream
    let input = io::stdin();

    // Get input lines as a vector of strings
    // see: https://doc.rust-lang.org/std/io/trait.BufRead.html
    let mut lines = input.lines()
        .map(|_line| _line.ok().unwrap())
        .collect::<Vec<String>>();          // Converts iterator into vector. Not necessary, see example solution.
    // The map acts on every element in the iterator, getting the value inside the returned Result, assuming the result is Ok(value) and not Err(error_message).
    // ok() returns an Option, so we unwrap it to get the value inside.

 let amount_of_num: u32 = lines[0].trim().parse().unwrap();
    let mut numbers: Vec<u32> = lines[1].split(" ").filter_map(|number| number.trim().parse::<u32>().ok())
    .collect::<Vec<u32>>();
   
    numbers.sort();
    let sum: u32 = numbers[(amount_of_num/2)as usize..].iter().sum();
 
    println!("{}", sum);

    eprintln!("Kattis skips this comment!");
    //println!("Print to standard output.");
   /* 
     //let mut input: [i32; AMOUNT] = [1,14,67,83,42,6,17,33,91];
     let mut input1: String = String::new();
     let mut input2: String  = String::new();

    io::stdin().read_line(&mut input1).expect("failed to read");

    let amount_of_num: usize= input1.trim().parse().unwrap();

    io::stdin().read_line(&mut input2).expect("failed to read");
    let mut numbers: Vec<u32> = Vec::new();

   
    for word in input2.split_whitespace() {

        let NumberToAdd = word.trim().parse().unwrap();
        numbers.push(NumberToAdd);
        
    }

    let mut sum = 0;

    numbers.sort();
    for i in 0..amount_of_num{
        if !(i < amount_of_num/2) {
            sum += numbers[i];
        }
        
    }

    println!("the sum is {}", sum);
    */
}
