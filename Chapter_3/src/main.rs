const X: i32 = 5;

fn main() {
    let x = 5;
    println!("Here is x: {}", x);
    let x = 6;
    println!("Here is x: {}", X);

    let spaces = "     ";
    let spaces = spaces.len();
    println!("Here is the number of spaces in spaces {} ", spaces,);

    let guess: u8 = "44".trim().parse().expect("Couldn't parse");
    println!("The guess is {}", guess);
    //floating points
    let a = 0.1;
    let b = 0.2;
    println!("{}",a+b);
}
