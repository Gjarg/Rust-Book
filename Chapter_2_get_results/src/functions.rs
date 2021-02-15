
use std::io::stdin;

pub fn less(max_val: i32, comp_number: i32) -> (i32, i32) {
    let mini = comp_number;
    let output = (max_val - comp_number) / 2 + comp_number;
    return (mini, output);
}

pub fn great(min_val: i32, comp_number: i32) -> (i32, i32) {
    let maxi = comp_number;
    let output = comp_number - (comp_number - min_val) / 2;
    return (maxi, output);
}

pub fn get_number() -> i32 {
    let mut value = String::new();
    loop {
        println!("Please enter a number");

        value.clear();
        stdin()
            .read_line(&mut value)
            .expect("Could'nt read the line");
        match value.trim().parse() {
            Ok(n) => return n,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };
    }
}