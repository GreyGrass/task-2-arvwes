
use std::io;
fn main() {
    const AMOUNT: usize = 9;
     //let mut input: [i32; AMOUNT] = [1,14,67,83,42,6,17,33,91];
     let mut input = String::new();
     let mut input2 = String::new();

    io::stdin().read_line(&mut input).expect("failed to read");

    let input: usize = input;
     io::stdin().read_line(&mut input2).expect("failed to read");

    

    for i in 0..AMOUNT {
        
    }
    let mut sum = 0;
    println!("Hello, world!");
       
    input.sort();
    println!("{}", input[1]);
    println!("the sum is now {}", sum);
    for i in 0..AMOUNT{
        if !(i < AMOUNT/2) {
            sum += input[i];
        }
        
    }

    println!("the sum is {}", sum);
}
