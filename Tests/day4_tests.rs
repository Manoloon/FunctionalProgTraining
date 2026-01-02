mod day4_tests
{
    use funcProg::day4::{ParseErrors,first_even,parse_positive,double_first_even, 
        first_positive_even};

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
}