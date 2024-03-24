use rand::Rng;
use std::io;

fn main() {
    println!("Guess a Number: ");

    let secret_number = rand::thread_rng().gen_range(1_i32..100);
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You Guessed: {}", guess);
    println!("Your Secret Number is: {}", secret_number);
}
