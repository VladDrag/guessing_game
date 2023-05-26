use rand::{thread_rng, Rng};
use std::io;

fn main() {
    println!("Welcome to my guessing game!");
    println!("Please input your guess.");
    println!("Please type a positive number between 1 and 100!");

    let mut rng_gen = thread_rng();
    let secret = rng_gen.gen_range(1..100);

    for i in 0..8 {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let guess = input
            .trim_end()
            .parse::<u8>()
            .expect("Code broken at line 22 in main.rs.");

        println!("You guessed: {}", guess);
        if guess == secret {
            println!("You win!");
            break;
        } else {
            println!("Try again!");
            println!("You have {} tries left", 8 - i - 1);
            if i != 7 {
                println!("Please input your guess.");
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
        input.clear();
    }
}
