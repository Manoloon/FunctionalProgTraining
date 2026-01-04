// dia 1
pub fn double(n: i32) -> i32 {
    n * n
}
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}
pub fn clamp(n: i32, min: i32, max: i32) -> i32 {
    match n {
        n if n < min => min,
        n if n > max => max,
        n => n,
    }
}
#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn day1_double() {
        assert_eq!(double(2), 4);
    }
    #[test]
    fn day1_is_even() {
        assert!(is_even(4));
    }
    #[test]
    fn day1_is_even_not() {
        assert!(!is_even(99));
    }
    #[test]
    fn day1_clamp() {
        assert_eq!(clamp(3, 1, 10), 3);
        assert_eq!(clamp(10, 2, 9), 9);
        assert_eq!(clamp(23, 30, 40), 30);
    }
}
