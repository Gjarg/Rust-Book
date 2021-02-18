use std::io;
fn main() {
    let mut input = String::new();
    println!("Return the nth Fibonacci number, please enter n :");
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read line");
    let input: u128 = input
        .trim()
        .parse()
        .expect("Couldn't chamge the type of the variable ");
    println!("\n \n");
    println!("The number is {} and th fib is {}", input, fib(input));
}
fn fib(n: u128) -> u128 {
    // 0, 1, 2, 3, 5
    let mut count = 2;
    let mut nth_2: u128 = 0;
    let mut nth_1: u128 = 1;
    if n == 0 {
        return nth_2;
    } else if n == 1 {
        return nth_1;
    }
    let mut val: u128 = 0;
    while count <= n {
        val = nth_1 + nth_2;
        nth_2 = nth_1;
        nth_1 = val;
        count += 1;
        println!("nth_2:{}  nth_1:{}  val:{}", nth_2, nth_1, val);
        //return val;
    }
    val
}

// fn fib(n:u128)->u128{
//     if n==1 ||n==0{
//         return n;
//     } else{
//         return fib(n-1)+fib(n-2);
//     }

// }
// fn fib (n:u128)->u128{
// match n {
//     0=>0,
//     1=>1,
//     num=> fib(num-1)+fib(num-2),
// }
// }
