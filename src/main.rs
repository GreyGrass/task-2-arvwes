
use std::io;
fn main() {
     
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
    
}
