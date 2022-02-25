use tests_lib::operation;

fn do_the_thing() -> i32 {
    operation(3, 2)
}

fn main() {
    println!("{}", do_the_thing());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_the_thing() {
        assert_eq!(1, do_the_thing());
    }
}