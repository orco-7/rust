#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
    fn perimeter(&self) -> u32 {
        2*(self.width + self.length)
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        length: 50,
    };

    let rect2 = Rectangle{
        width: 20,
        length: 40,
    };

    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!(
        "the area of the Rectangle is {}\nits perimeter {}",
        rect1.area(), rect1.perimeter()
    );

    //println!("{:#?}", rect1);
}