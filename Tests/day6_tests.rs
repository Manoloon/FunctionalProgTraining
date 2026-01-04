mod day6_tests {
    use funcProg::day6::{first_drop, product, running_max};
    // Day 6
    #[test]
    fn day6_product() {
        let vals = [2, 3, 4];
        let result = 2 * 3 * 4;
        assert_eq!(product(&vals), Some(result));
    }
    #[test]
    fn day6_running_max() {
        let vals = [3, 1, 4, 2, 5];
        let res = [3, 3, 4, 4, 5];
        assert_eq!(running_max(&vals), res);
    }
    #[test]
    fn day6_first_drop() {
        let vals = [5, 7, 6, 9];
        assert_eq!(first_drop(&vals), Some(6));
    }
}
