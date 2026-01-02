mod day5_tests
{
    use funcProg::day5::{all_positive_fold,sum_fold,sum_of_squares_of_even,
        count_greater_than,first_negative,second_even};

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
    fn day5_first_negative()
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
}