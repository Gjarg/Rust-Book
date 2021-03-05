#[derive(Debug)]
enum Game{
    Name(String),
}
impl Game{
    fn create(name:  &str)-> Game{
        Game::Name(name.into())
    }

}


fn main() {
let mario = Game::create("Mario Brothers");
println!("{:?}",mario);
let mut number:Option<u32>= None;
println!("Number is {:?}",&number);
number = Some(25);
println!("Number is {:?},{}",number,number.unwrap());
}
