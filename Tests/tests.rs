mod recursive_tests
{
    use funcProg::{double,is_even,clamp,
        factorial,all_positive,length,sum,count_positive,
        sum_of_even_squares,ParseErrors,
        first_even,parse_positive,double_first_even, 
        first_positive_even,
        all_positive_fold,sum_fold,sum_of_squares_of_even,count_greater_than,first_negative,second_even,
        product,running_max,first_drop};

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
    // Day 3
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
    // Day 4
    #[test]
    fn day4_first_even()
    {
        let values = [7,9,2,3,-3,1];
        assert_eq!(first_even(&values),Some(2));
    }
    #[test]
    fn day4_first_even_negative()
    {
        let values = [7,-4,2,3,-3,1];
        assert_eq!(first_even(&values),Some(-4));
    }
    #[test]
    fn day4_parse_positive_ok() {
        assert_eq!(parse_positive("42"), Ok(42));
    }
    #[test]
    fn day4_parse_positive_zero() {
        assert_eq!(parse_positive("0"), Err(ParseErrors::NotPositive));
    }
    #[test]
    fn day4_parse_positive_negative() {
        assert_eq!(parse_positive("-3"), Err(ParseErrors::NotPositive));
    }
    #[test]
    fn day4_parse_positive_invalid() {
        assert_eq!(parse_positive("abc"), Err(ParseErrors::NotANumber));
    }
    #[test]
    fn day4_double_first_even()
    {
        let values = [7,9,2,3,-3,1];
        assert_eq!(double_first_even(&values),Some(4));
    }
    #[test]
    fn day4_first_positive_even()
    {
        let values = [-4,9,2,3,-3,1];
        assert_eq!(first_positive_even(&values),Some(2));
    }
    #[test]
    fn day4_first_negative()
    {
        let values = [10,3,4,-4,9,2,3,-3,1];
        assert_eq!(first_negative(&values),Some(-4));
    }

    #[test]
    fn day4_second_even()
    {
        let values = [10,3,4,-4,9,2,3,-3,1];
        assert_eq!(second_even(&values),Some(4));
    }
    // Day 5
    #[test]
    fn day5_all_positive()
    {
        let values = [10,3,4,-4,9,2,3,-3,1];
        assert!(!all_positive_fold(&values));
        let val2 = [10,2,4];
        assert!(all_positive_fold(&val2));
    }
    #[test]
    fn day5_sum_fold()
    {
        let values = [10,3,4,-4,9,2,3,-3,1];
        let res = 10+3+4+-4+9+2+3+-3+1;
        assert_eq!(sum_fold(&values),res);
    }
    #[test]
    fn day5_sum_of_squares_of_even()
    {
        let vals = [1,103,10,2,4];
        let res = 10*10+2*2+4*4;
        assert_eq!(sum_of_squares_of_even(&vals),res);
    }
    #[test]
    fn day5_count_greater_than()
    {
        let vals = [1,103,10,2,4];
        assert_eq!(count_greater_than(&vals,10),1);
    }
    #[test]
    fn day_first_negative()
    {
        let values = [10,3,4,-4,9,2,3,-3,1];
        assert_eq!(first_negative(&values),Some(-4));
    }
    #[test]
    fn day5_second_even()
    {
       let values = [10,3,4,-4,9,2,3,-3,1];
       assert_eq!(second_even(&values),Some(4)); 
    }
    // Day 6 
    #[test]
    fn day6_product()
    {
        let vals = [2,3,4];
        let result = 2 * 3 * 4;
        assert_eq!(product(&vals),Some(result));
    }
    #[test]
    fn day6_running_max()
    {
        let vals = [3, 1, 4, 2, 5];
        let res = [3,3,4,4,5];
        assert_eq!(running_max(&vals),res);
    }
    #[test]
    fn day6_first_drop()
    {
        let vals = [5,7,6,9];
        assert_eq!(first_drop(&vals),Some(6));
    }
}