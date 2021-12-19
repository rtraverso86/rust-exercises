use std::io::{self, Write};

mod organization;
mod parser;

use organization::Organization;
use parser::Command;

fn prompt_user_input() -> String {
    print!("> ");
    io::stdout().flush().expect("failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    input.trim().to_string()
}

fn run_command(org: &mut Organization, cmd: Command) {
    match cmd {
        Command::Quit => {
            std::process::exit(0);
        }
        Command::ListAll => {
            println!();
            org.print_all();
        }
        Command::ListDept { dept } => {
            println!();
            org.print_list(&dept);
        }
        Command::Add { person, dept } => {
            org.add(person, dept);
        }
        Command::Help => {
            println!("\nAvailable commands:");
            println!("  - Add <Name> to <Department>");
            println!("  - List employees");
            println!("  - List employees of <Department>");
            println!("  - Help");
            println!("  - Quit");
        }
    };
}

fn main() {
    let mut org = Organization::new();
    loop {
        if let Some(cmd) = parser::parse(&prompt_user_input()) {
            run_command(&mut org, cmd);
        } else {
            println!("|  No such command");
        }
        println!();
    }
}
