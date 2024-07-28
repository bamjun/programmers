use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let nums: Vec<i32> = input.split_whitespace().map(|I| I.parse().unwrap()).collect();
    
    let difference = nums[0] - nums[1];
    
    println!("{}", difference);
}