use std::io::{self, Write};

fn is_vowel(c: char) -> bool {
    ['a', 'e', 'i', 'o', 'u'].contains(&c.to_lowercase().next().unwrap())
}

fn pig_latin(sentence: &str) -> String {
    let mut res = String::with_capacity(sentence.len());

    for word in sentence.split_whitespace() {
        let mut char_iter = word.chars();
        if let Some(initial) = char_iter.next() {
            if is_vowel(initial) {
                res = res + word + "-hay ";
            } else {
                let remaining: String = char_iter.collect();
                res = res + &remaining + &format!("-{}ay ", initial);
            }
        }
    }

    res
}

fn main() {
    loop {
        print!("Enter the sentence to translate to pig-latin.\n> ");
        io::stdout().flush().expect("failed to write to stdout");

        let mut input_sentence = String::new();
        io::stdin()
            .read_line(&mut input_sentence)
            .expect("failed to read from stdin");
        input_sentence.truncate(input_sentence.trim_end().len()); // trim in place

        if input_sentence.is_empty() {
            break;
        }
        println!("{}\n", pig_latin(&input_sentence));
    }
}
