use std::io;

fn fahrenheit_to_celsius(f : f32) -> f32 {
    ((f - 32.0) / 9.0) * 5.0
}

fn celsius_to_fahrenheit(c : f32) -> f32 {
    (c * 9.0 / 5.0) + 32.0
}

fn parse_temp(t : &str) -> (char, f32) {
    let t = t.trim().to_uppercase();
    for suffix in ["°F", "°C"] {
       match t.strip_suffix(suffix) {
            Some(x) => return (suffix.chars().last().unwrap(), x.trim_end().parse().expect("NaN")),
            None    => continue
        }
    }
    ('\0', 0.0)
}

fn convert_temp(tuple : (char, f32)) -> (char, f32) {
    match tuple {
        ('F', v)  => ('C', fahrenheit_to_celsius(v)),
        ('C', v)  => ('F', celsius_to_fahrenheit(v)),
        x => x
    }
}

fn main() {
    loop {
        println!("Please enter a °C or °F value for conversion:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input == "\n" { break }
        let (u, v) = convert_temp(parse_temp(&input));
        if u == '\0' { continue }
        println!("The converted temperature is {}°{}", v, u);
    }
}
