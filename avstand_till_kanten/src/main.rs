use std::io::{self, stdout};

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
