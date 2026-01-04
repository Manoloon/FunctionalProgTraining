mod day7_tests {
    use funcProg::day7::{Fibonacci, running_product, running_sum,running_max,drop_until};

    #[test]
    fn fibonacci_correcto() {
        let result: Vec<i32> = Fibonacci::new(20).collect();
        let require = [0, 1, 1, 2, 3, 5, 8, 13];
        assert_eq!(result, require);
    }
    #[test]
    fn fibo_limite_exacto() {
        let result: Vec<i32> = Fibonacci::new(13).collect();
        assert_eq!(result.last(), Some(&13));
    }
    #[test]
    fn fibo_limit_zero() {
        let result: Vec<i32> = Fibonacci::new(0).collect();
        assert_eq!(result, vec![0]);
    }
    #[test]
    fn fibo_limit_negative() {
        let result: Vec<i32> = Fibonacci::new(-1).collect();
        assert!(result.is_empty());
    }
    #[test]
    fn fibonacci_partial_consumption() {
        let fib = Fibonacci::new(100);
        let first_three: Vec<i32> = fib.take(3).collect();
        assert_eq!(first_three, vec![0, 1, 1]);
    }

    #[test]
    fn RunningSum_correcto() {
        let data = vec![1, 2, 3, 4];
        let require = vec![1, 3, 6, 10];
        let result: Vec<i32> = running_sum(data.into_iter()).collect();
        assert_eq!(result, require);
    }
    #[test]
    fn running_sum_composable() {
        let v = vec![1, -1, 2];
        let out: Vec<i32> = running_sum(v.into_iter()).filter(|x| *x > 0).collect();
        assert_eq!(out, vec![1, 2]);
    }

    #[test]
    fn RunningProduct_correcto() {
        let data = vec![1, 2, 3, 4];
        let require = vec![1, 2, 6, 24];
        let result: Vec<i32> = running_product(data.into_iter()).collect();
        assert_eq!(result, require);
    }

    #[test]
    fn RunningMax_correcto() {
        let data = vec![3, 1, 5, 2];
        let require = vec![3, 3, 5, 5];
        let result: Vec<i32> = running_max(data.into_iter()).collect();
        assert_eq!(result, require);
    }

    #[test]
    fn DropUntil_correcto()
    {
        let data = vec![-3,-2,0,4,5];
        let require = vec![4,5];
        let result: Vec<i32> = drop_until(data.into_iter()).collect();
        assert_eq!(result,require);
    }
}
