extern crate rand;

use std::io;
use rand::Rng;

fn get_secret_number() -> i32 {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("secret is {}", secret_number);
    return secret_number;
}
fn guess_game() {
    let sec_num = get_secret_number();
    
    println!("guess a number!");
    println!("please input your guess");

    
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("you guessed {}", guess); 
}
fn main() {
//     let numbers = [1, 2, 5].iter();

//     let sum = numbers
//         .fold(0, |acc, next| acc+next);
    
//     println!("sum is {:#?}", sum);

    guess_game();
}
