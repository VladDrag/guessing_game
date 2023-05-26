use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to my guessing game!");
    println!("Please take a guess and type a positive number between 1 and 100!");

    let mut rng_gen = thread_rng();
    let secret = rng_gen.gen_range(1..100);

    for i in 0..8 {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a positive number between 1 and 100!");
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Less => {
                println!("Too small!");
                println!("You have {} guesses left!", 8 - i - 1)
            }
            Ordering::Greater => {
                println!("Too big!");
                println!("You have {} guesses left!", 8 - i - 1)
            }
        }
    }
}
