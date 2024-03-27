use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a Number: ");

    let mut count: i8 = 0;

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type of a number!");

        if count < 10 {
            match guess.cmp(&secret_number) {
                Ordering::Less => {
                    println!("Too Small");
                    count = count + 1;
                    println!("Only {} chances left!", 10 - count);
                }
                Ordering::Greater => {
                    println!("Too Big");
                    count = count + 1;
                    println!("Only {} chances left!", 10 - count);
                }
                Ordering::Equal => {
                    println!("You win!");
                    println!("You took {} chances to win!", count);
                    break;
                }
            }
        } else {
            print!("You Loose!")
        }
    }
}
