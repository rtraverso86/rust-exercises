#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Associated functions
impl Rectangle {
    // These three are specifically methods
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other : &Rectangle) -> bool {
        (self.width >= other.width && self.height >= other.height)
            || (self.width >= other.height && self.height >= other.width)
    }
    // This is a plain associated function used as a constructor, like String::from()
    fn square(size : u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(15 * scale),
        height: 50,
    };
    println!(
        "rect1 is {:?} and its area is {} square pixels.",
        rect1,
        rect1.area() //area(&rect)
    );
    if rect1.width() {
        println!("rect1 has non-zero width; it is {}.", rect1.width);
    }
    dbg!(&rect1);

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    dbg!(Rectangle::square(5));
}

fn area(rect : &Rectangle) -> u32 {
    rect.width * rect.height
}
