use rand::{thread_rng, Rng};
use std::cmp::Ordering;
mod functions;

fn main() {
    let mut min_possible_secret: i32 = 1;
    let mut max_possible_secret: i32 = 100;
    let my_number: i32 = functions::get_number();
    let mut computer_number: i32 = thread_rng().gen_range(1..101);
    println!("This is the computer number: {}", computer_number);

    loop {
        match computer_number.cmp(&my_number) {
            Ordering::Less => {
                println!("Too small");
                let val = functions::less(max_possible_secret, computer_number);
                min_possible_secret = val.0;
                computer_number = val.1;
                println!("Here is the computer guess: {}", computer_number);
            }
            Ordering::Greater => {
                println!("Too big");
                let val = functions::great(min_possible_secret, computer_number);
                max_possible_secret = val.0;
                computer_number = val.1;
                println!("Here is the computer guess: {}", computer_number);
            }
            Ordering::Equal => {
                println!("Great you won");
                break;
            }
        };
    }
}

