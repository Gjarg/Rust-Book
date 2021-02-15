use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::{fs::File, io::Write};
mod functions;

fn main() {
    let my_number: i32 = functions::get_number();
    let mut i: i32 = 0;

    let mut file = File::create("results.txt").expect("Couldn't create the file");
    for _k in 0..10 {
        let mut min_possible_secret: i32 = 1;
        let mut max_possible_secret: i32 = 10000;
        let mut computer_number: i32 = thread_rng().gen_range(min_possible_secret, max_possible_secret + 1);
        let mut total_val = String::new();
        total_val.push_str(&format!("--------------------------------------\n **--** Computer starts at {}\tthe number to discover is {}\n",
        &computer_number, &my_number));
        loop {
            match computer_number.cmp(&my_number) {
                Ordering::Less => {
                    let val = functions::less(max_possible_secret, computer_number);
                    min_possible_secret = val.0;
                    computer_number = val.1;
                }
                Ordering::Greater => {
                    let val = functions::great(min_possible_secret, computer_number);
                    max_possible_secret = val.0;
                    computer_number = val.1;
                }
                Ordering::Equal => {
                    break;
                }
            };
            total_val.push_str(&format!("{}) Computer guess {}\n", &i, &computer_number));
            i += 1;
        }
        file.write_fmt(format_args!("{}", total_val))
            .expect("Couldn't right safely");
        i = 0;
    }
}
