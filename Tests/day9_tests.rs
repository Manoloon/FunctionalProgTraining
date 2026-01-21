mod day9_tests
{
    use funcProg::day9::{repeat_value,running_sum_until};

    #[test]
    fn count_infinite()
    {
        let require = vec![7,7,7,7];
        let result: Vec<i32> = repeat_value(7).take(4).collect();
        assert_eq!(result, require);
    }
    #[test]
    fn running_sum_until_basic() {
        let data = vec![3, 4, 5];
        let result: Vec<i32> = running_sum_until(data.into_iter(), 6).collect();
        assert_eq!(result, vec![3, 7]);
    }
    #[test]
    fn running_sum_until_never_exceeds() {
        let data = vec![1, 2, 3];

        let result: Vec<i32> = running_sum_until(data.into_iter(), 10).collect();

        assert_eq!(result, vec![1, 3, 6]);
    }
    #[test]
    fn running_sum_until_first_exceeds() {
        let data = vec![10, 1, 1];

        let result: Vec<i32> = running_sum_until(data.into_iter(), 5).collect();

        assert_eq!(result, vec![10]);
    }
    #[test]
    fn running_sum_until_empty() {
        let data: Vec<i32> = vec![];

        let result: Vec<i32> = running_sum_until(data.into_iter(), 5).collect();

        assert!(result.is_empty());
    }
    #[test]
    fn running_sum_until_is_fused() {
        let mut iter = running_sum_until(vec![3, 4, 5].into_iter(), 6);

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}