pub fn is_odd(x: usize) -> bool {
    x % 2 == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd() {
        assert!(is_odd(1));
    }

    #[test]
    fn test_even() {
        assert!(!is_odd(2));
    }

    #[test]
    fn test_zero() {
        assert!(!is_odd(0));
    }
}
