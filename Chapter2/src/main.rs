use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcom to the game: Guest the number");
    let secret_number = thread_rng().gen_range(0, 100);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number");
                continue;
            },
        };

        println!(
            "Your guess: {}, the secret number is {} ",
            guess, secret_number
        );
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win !");
                break;
            }
        };
    }
}
