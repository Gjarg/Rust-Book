fn main() {
    let s1 = String::from("Hello");
    prt(&s1);
    let s2 = &s1;
    println!("{},{}", s1,s2);
}

fn prt(st : &str){
    println!("{}",st);
}