fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "nineth",
        "tenth", "eleventh", "twelth",
    ];
    let lyrics = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    let mut count = 1;


    for day in &days {
        println!("On the {} day of Christmas, my true love sent to me", day);
        for i in (0..count).rev(){
            println!("{}", lyrics[i]);
        }count += 1;println!("\n");
    }
}