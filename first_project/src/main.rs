extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn get_secret_number() -> u32 {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("secret is {}", secret_number);
    secret_number
}

fn get_guess() -> u32 {
    let mut guess = String::new();

    println!("please input your guess");
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = guess.trim().parse()
        .expect("Please type a positive integer!");
    guess
}

fn compare_guess(guess:u32, sec_num:u32) -> String {
    let result =
        match guess.cmp(&sec_num) {
            Ordering::Less => "too small",
            Ordering::Greater => "too large",
            Ordering::Equal => "you got it",
        };
    result.to_string()
}
fn guess_game() {
    println!("guess a number!");
    
    let guess = get_guess();
    let sec_num = get_secret_number();
    
    println!("you guessed {}", guess); 

    let answer = compare_guess(guess, sec_num);
    println!("{}", answer)

}
fn main() {

    guess_game();
}
