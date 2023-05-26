use rand::{thread_rng, Rng};
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

        let guess = guess
            .trim()
            .parse::<i32>()
            .expect("Code broken at line 22 in main.rs.");

        if guess == secret {
            println!("You win!");
            break;
        } else {
            println!("Try again!");
            println!("You have {} tries left", 8 - i - 1);
            if i != 7 {
                println!("Please guess your guess.");
            } else {
                println!("You lose!");
                break;
            }
        }

        if guess < secret {
            println!("Too small!");
        }

        if guess > secret {
            println!("Too big!");
        }
    }
}
