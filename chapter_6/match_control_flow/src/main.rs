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

fn coin_value(coin: &Coin) -> u32 {
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

fn add_coin_value(cents: u32, total_value: Option<i32>) -> Option<i32> {
    match total_value {
        None => None,
        Some(current_cents) => Some(cents as i32 + current_cents),
    }
}
fn get_value_coin (opt_coin : Option<i32>)-> i32{
    opt_coin.unwrap()
}

fn main() {
    let total_value = Some(0);
    let pen = Coin::Penny;
    let dim = Coin::Dime;
    let quat = Coin::Quater(UsState::Colorado);
    let cc = coin_value(&quat);
    println!(
        "The value of a Penny is {:?}, dime is{:?}, quater is {:?}",
        coin_value(&pen),
        coin_value(&dim),
        &cc
    );
    let total_value = add_coin_value(coin_value(&pen), total_value);
    let total_value = add_coin_value(coin_value(&dim), total_value);
    let total_value = add_coin_value(coin_value(&quat), total_value);
    println!("Total coins value {} cents.",get_value_coin(total_value))
}
