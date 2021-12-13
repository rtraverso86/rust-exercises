use Message::*; // avoid using Message::XYZ

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Quit => println!("Quitting."),
            Move {x, y} => println!("Moved to ({}, {}).", x, y),
            Write(s) => println!("Message: {}", s),
            ChangeColor(r, g, b) => println!("New RGB colour: ({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    let sequence : [Message; 4] = [
        Write(String::from("Hello!")),
        ChangeColor(0, 0, 200),
        Move { x: 5, y: 0 },
        Quit,
    ];
    sequence.map(|m| m.call());
}
