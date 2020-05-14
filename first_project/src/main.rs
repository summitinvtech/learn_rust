extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    
    println!("guess a number!");
    println!("please input your guess");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("you guessed {}", guess);
}
