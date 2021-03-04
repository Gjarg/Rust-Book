fn main() {
    let s1 = String::from("Hello");
    let a = prt(s1);
    let s2 = &a;
    println!("{},{}", a, s2);
    let new_st = String::from("Coucou");
    let len_new_string = calc_len(&new_st);
    println!("{} has a length of {}", new_st, len_new_string);
    let mut s = String::from("Hello ");
    {
        let _s1 = &mut s;
    }
    let s2 = &mut s;
    println!("{}", s2);
    println!("{}" ,b' ');
    let hello = "hello world!";
    let h = &hello[0..5];
    let all= &hello[..];
    println!("{}{}",hello,h);
    println!("{}",all);

    let my_ = [0,1,2,3];
    println!("{:?}",my_);
}





fn prt(st: String) -> String {
    println!("{}", st);
    st
}
fn calc_len(st: &str) -> usize {
    let a = st.len();
    a
}
