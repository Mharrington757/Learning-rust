use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret number is {secret_number");

    println!("Please input your guess.");
     
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    match guess.cmp(&secret_number) {
        Ordering::Less => ("Too small"),
        Ordering::Greater => ("Too big"),
        Ordering::Equal => ("You win"),
    };

    println!("Your guessed: {guess}");
}
