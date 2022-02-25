pub fn operation(x: i32, y: i32) -> i32 {
    x - y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation_test() {
        assert_eq!(4, operation(6, 2));
        assert_eq!(-4, operation(2, 6));
    }
}