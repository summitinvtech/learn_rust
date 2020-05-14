extern crate rand;

use std::io;
use rand::Rng;

fn guess_game() {
    println!("guess a number!");
    println!("please input your guess");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("secret is {}", secret_number);
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("you guessed {}", guess); 
}
fn main() {
    let numbers = [1, 2, 5].iter();

    let sum = numbers
        .fold(0, |acc, next| acc+next);
    
    println!("sum is {:#?}", sum);
}
