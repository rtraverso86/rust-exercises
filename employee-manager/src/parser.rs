// LANGUAGE:
//   Lang ::= {0} (Add | List | Quit) '\n' {1}
//   Quit ::= 'quit' {2}
//   Add ::= 'add' {3} Name 'to' {4} Name
//   List ::= 'list' {5} 'employees' {6} 'of' {7} Name
//   Helo ::= 'help' {8}
//   Name ::= <every sequence of word different from 'to'>

#[derive(Debug)]
pub enum Command {
    Add {person: String, dept: String},
    List {dept: String},
    Quit,
    Help,
    NoSuchCommand,
}

fn token_id(token: &str) -> i8 {
    match token {
        "quit" => 0,
        "add" => 1,
        "to" => 2,
        "list" => 3,
        "employees" => 4,
        "of" => 5,
        "\n" => 6,
        "help" => 8,
        _ => 7,
    }
}

fn append_token(to: &mut String, what: &str) {
    if !to.is_empty() {
        to.push_str(" ");
    }
    to.push_str(what);
}


pub fn parse(input: &str) -> Command {
    let mut state : i8 = 0;
    let mut cmd_id : i8 = -1;
    let mut arg1 = String::new();
    let mut arg2 = String::new();
    let table = [
        // 0   1   2   3   4   5   6   7: tokens
        [  2,  3, -1,  5, -1, -1, -1, -1,  8], // state 0
        [ -1, -1, -1, -1, -1, -1, -1, -1, -1], // state 1 accepting
        [ -1, -1, -1, -1, -1, -1,  1, -1, -1], // state 2 (quit)
        [ -1, -1,  4, -1, -1, -1, -1,  3, -1], // state 3 (add)
        [ -1, -1, -1, -1, -1, -1,  1,  4, -1], // state 4 (to)
        [ -1, -1, -1, -1,  6, -1, -1, -1, -1], // state 5 (list)
        [ -1, -1, -1, -1, -1,  7, -1, -1, -1], // state 6 (employees)
        [ -1, -1, -1, -1, -1, -1,  1,  7, -1], // state 7 (of)
        [ -1, -1, -1, -1, -1, -1,  1, -1,  8], // state 8 (help)
    ];
    for tok in input.split_whitespace() {
        let tok_l = tok.to_lowercase();
        let tok_id = token_id(&tok_l);
        if tok_id == 7 {
            if state == 3 || state == 7 {
                append_token(&mut arg1, tok);
            } else if state == 4 {
                append_token(&mut arg2, tok);
            }
        }
        println!("|    State: {}\tTok: {} / {}", state, tok_id, tok_l);
        state = table[state as usize][tok_id as usize];
        println!("|      => New State: {}", state);
        if state < 0 {
            return Command::NoSuchCommand;
        }
        match tok_id {
            0 | 1 | 3 | 8 => { cmd_id = tok_id; },
            _ => (),
        }
    }
    state = table[state as usize][token_id("\n") as usize];
    if state == 1 {
        println!("|    ACCEPTED");
        let cmd = match cmd_id {
            0 => Command::Quit,
            1 => Command::Add { person: arg1, dept: arg2 },
            3 => Command::List { dept: arg1 },
            8 => Command::Help,
            _ => Command::NoSuchCommand,
        };
        println!("|  Command: {:?}", cmd);
        return cmd;
    }
    println!("|    NOT ACCEPTED");
    Command::NoSuchCommand
}

