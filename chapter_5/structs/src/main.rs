fn main() {
    struct MyStruct {
        name: String,
        age: u32,
    }

    let snoop = MyStruct {
        name: String::from("Kiki"),
        age: 5,
    };
    println!("name is {} has {}", snoop.name, snoop.age);

    let black = Color(0, 0, 0);
    let all_are_one = convert_to_one(&black);
    println!("first color is {} and the second is {}", black.0, black.1);
    println!("{:?}", all_are_one)
}
struct Color(i32, i32, i32);
fn convert_to_one(_t: &Color) -> (i32, i32, i32) {


    (1,1,1)
}
