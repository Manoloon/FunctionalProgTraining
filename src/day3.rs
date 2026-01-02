// Day 3
pub fn count_positive(slice: &[i32]) -> usize
{
    slice.iter().filter(|&x| *x > 0).count()
}

pub fn sum_of_even_squares(slice: &[i32]) -> i32
{
    slice.iter().filter(|&x| x % 2 == 0).map(|x| x * x).sum()
}

// Reescribe tu función length usando iteradores (sin recursión).
#[cfg(test)]
mod day3_tests{
    use super::*;

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