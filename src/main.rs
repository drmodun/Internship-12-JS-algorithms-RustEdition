
use std::io::*;
use rand::Rng;
fn main() {
    println!("What is your name");
    let mut name = String::new();
    let greeting = "Hello";
    stdin().read_line(&mut name).expect("Failed to read line");

    println!("Hello, world {}", name.trim_end());
}

fn task1(){
    
}