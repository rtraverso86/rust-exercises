use std::io::{self, Write};

mod parser;
mod organization;

use parser::Command;
use organization::Organization;

fn prompt_user_input() -> String {
    print!("> ");
    io::stdout().flush().expect("failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    input.trim().to_string()
}

fn main() {
    let mut org = Organization::new();
    loop {
        let cmd = parser::parse(&prompt_user_input());
        match cmd {
            Command::Quit => {
                return;
            },
            Command::List{dept} => {
                println!("");
                org.print_list(&dept);
            },
            Command::Add{person, dept} => {
                org.add(person, dept);
            },
            Command::Help => {
                println!("\nAvailable commands:");
                println!("  - Add <Name> to <Department>");
                println!("  - List employees of <Department>");
                println!("  - Help");
                println!("  - Quit");
            },
            Command::NoSuchCommand => {
                println!("|  No such command");
            },
        }
        println!("");
    }
}
