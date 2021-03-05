enum Coin {
    Penny,
    Quater(UsState),
    Dime,
    Nickel,
}
#[derive(Debug)]
enum UsState {
    Colorado,
    Alabama,
}

fn coin_value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Quater(state) => {
            println!("This quarter is form {:?}", state);
            25
        }
        Coin::Dime => 10,
        Coin::Nickel => 5,
    }
}

fn main() {
    let pen = Coin::Penny;
    let dim = Coin::Dime;
    let quat = Coin::Quater(UsState::Colorado);
    let cc = coin_value(quat);
    println!(
        "The value of a Penny is {:?}, dime is{:?}, quater is {:?}",
        coin_value(pen),
        coin_value(dim),
        &cc
    )
}