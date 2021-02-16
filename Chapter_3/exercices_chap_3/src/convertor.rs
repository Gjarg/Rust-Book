use std::io;

pub fn conv() {
    println!("Convert degres Celsius in Fahrenheit");
    let val = get_temp();
    let symbol = get_symbol();
    if symbol == "c" {
        println!("{} degres C correspond to {} degres F", val, c_to_f(val))
    } else {
        println!("{} degres F correspond to {} degres C", val, f_to_c(val))
    }
}

pub fn get_temp() -> f32 {
    let mut input = String::new();
    println!("What is the temp that you want to convert?");
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read the line");
    return input
        .trim()
        .parse()
        .expect("Couldn't change the type of the variable");
}

pub fn get_symbol() -> std::string::String {
    let mut input = String::new();
    println!("Is the temp in C or F ?");
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read the line");
    let temp_symbol = input.trim().to_string();

    if temp_symbol == "f" || temp_symbol == "c" {
        temp_symbol
    } else {
        panic!("Please enter C or F.")
    }

}
pub fn c_to_f(x: f32) -> f32 {
    (x * 9.0 / 5.0) + 32.0
}
pub fn f_to_c(x: f32) -> f32 {
    (x - 32.0) * 5.0 / 9.0
}
