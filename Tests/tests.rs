mod recursive_tests
{
    use funcProg::{double,is_even,clamp,factorial,all_positive,length,sum,count_positive,sum_of_even_squares};

    #[test]
    fn day1_double() { assert_eq!(double(2),4); }
    #[test]
    fn day1_is_even() { assert!(is_even(4));}
    #[test]
    fn day1_is_even_not() {assert!(!is_even(99));}
    #[test]
    fn day1_clamp() { 
        assert_eq!(clamp(3, 1, 10),3);
        assert_eq!(clamp(10, 2, 9),9);
        assert_eq!(clamp(23,30,40),30);
    }
    // DAY 2
    #[test]
    fn day2_factorial() {assert_eq!(factorial(4),24)}
    #[test]
    fn day2_sum() {
        let values = [1, 2, 3, 4];
        let result = sum(&values);
        assert_eq!(sum(&values),result);
    }
    #[test]
    fn day2_str_length()
    {
        let test = "abc";
        assert_eq!(length(test),3);
    }
    #[test]
    fn day2_all_possitive()
    {
        let values = [1, 2, 3, 4];
        assert!(all_positive(&values));
        let values2 = [1, -2, 3, 4];
        assert!(!all_positive(&values2));
    }

    #[test]
    fn day3_count_positive()
    {
        let values = [2,3,-3,1];
        assert_eq!(count_positive(&values),3);
    }
    #[test]
    fn day3_sum_of_even_squares()
    {
        let values = [2,3,-3,1];
        assert_eq!(sum_of_even_squares(&values),4);
    }
}