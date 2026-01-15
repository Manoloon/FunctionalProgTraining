mod day8_tests
{
    use funcProg::day8::{diff_hide,emit_until};

    #[test]
    fn diff_correct()
    {
        let input = vec![5,8,10,7];
        let require = vec![3,2,-3];
        let result: Vec<i32> = diff_hide(input.into_iter()).collect();
        assert_eq!(result, require);
    }

    #[test]
    fn emit_until_correcto()
    {
        let input = vec![1,2,3,2,5];
        let require = vec![1,2,3];
        let result: Vec<i32> = emit_until(input.into_iter()).collect();
        assert_eq!(result, require);
    }
    #[test]
    fn emit_until_single() {
        let data = vec![10];
        let result: Vec<i32> = emit_until(data.into_iter()).collect();
        assert_eq!(result, vec![10]);
    }
}