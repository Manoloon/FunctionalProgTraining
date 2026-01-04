mod day2_tests {
    use funcProg::day2::{all_positive, factorial, length, sum};

    #[test]
    fn day2_factorial() {
        assert_eq!(factorial(4), 24)
    }
    #[test]
    fn day2_sum() {
        let values = [1, 2, 3, 4];
        let result = sum(&values);
        assert_eq!(sum(&values), result);
    }
    #[test]
    fn day2_str_length() {
        let test = "abc";
        assert_eq!(length(test), 3);
    }
    #[test]
    fn day2_all_possitive() {
        let values = [1, 2, 3, 4];
        assert!(all_positive(&values));
        let values2 = [1, -2, 3, 4];
        assert!(!all_positive(&values2));
    }
}
