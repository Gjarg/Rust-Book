#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn fit_in(&self, r: &Rectangle) -> bool {
        self.height < r.height && self.width < r.width
    }
     fn square(size:u32)-> Rectangle{
         Rectangle{
             width:size,
             height:size,
         }
     }
}

fn calculate_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn main() {
    let rect1 = Rectangle {
        width: 31,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 30,
        height: 45,
    };
    let square1 = Rectangle::square(50);
    let area = calculate_area(&rect1);
    let area2 = rect1.area();
    let fit12 = rect1.fit_in(&rect2);
    let fit21 = rect2.fit_in(&rect1);
    println!(
        "The area is {}, with a height of {} and a width of {}, my rectangle {:#?}.",
        area, rect1.height, rect1.width, rect1
    );
    println!("area method {}", area2);
    println!("should be false {}, should be true {}", fit12, fit21);
    println!("It is the area of square1 {} with a side size of {}",calculate_area(&square1),square1.height)
}
