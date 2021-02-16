const X: i32 = 5;

fn main() {
    let x = 5;
    println!("Here is x: {}", x);
    let x = 6;
    println!("Here is x: {}{}", X, x);

    let spaces = "     ";
    let spaces = spaces.len();
    println!("Here is the number of spaces in spaces {} ", spaces,);

    let guess: u8 = "44".trim().parse().expect("Couldn't parse");
    println!("The guess is {}", guess);
    //floating points
    let a = 0.1;
    let b = 0.2;
    println!("{}", a + b);
    //booleans
    let _vrai: bool = true;
    //tuple
    let my_collection = (15, "hello", '8');
    let (my_num, my_word, alphabet) = my_collection;
    println!("{}, {}, {}, {}", my_num, my_word, alphabet, my_collection.0);
    //array type
    let arr = [1, 2, 3, 4, 5];
    let index = 1;
    println!("Voici {} in the place {}", arr[index], index);

    //functions
    let player_position = (15, 12);
    another_function(5);
    let y = test_tuple(player_position);

    println!("{}", y);

    //condition

    let arrays = [1,2,3,4,5,6,7,8,9];
    for i in arrays.iter(){
        println!("{}",i)
    }
}

fn another_function(x: i32) {
    println!(
        "This is from another function and prints this number {}.",
        x
    );
}
fn test_tuple(tup: (i32, i32)) -> i32 {
    let (x, y) = tup;
    println!("Player position x:{},y:{}", x, y);
    return x;
}
