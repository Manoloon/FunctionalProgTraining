// DAY 5 Fold
pub fn all_positive_fold(slice: &[i32])->bool
{
    slice.iter().fold(true, |acc,&x| acc && x > 0)
}
pub fn sum_fold(slice: &[i32]) ->i32
{
    slice.iter().fold(0,|acc,x,| acc + x )
}

pub fn sum_of_squares_of_even(slice: &[i32]) -> i32
{
    slice.iter().fold(0,|acc,&x| 
        {
            if x % 2 == 0
            {
                acc + (x * x)
            }
            else 
            {
                acc
            }
        })
}
// Cuenta los valores mayores que limit.
pub fn count_greater_than(slice: &[i32], limit: i32) -> usize
{
    slice.iter().fold(0, |acc,&x| acc + (x > limit) as usize)
}
    /*
    Devuelve el primer número negativo.
    Reglas:
    Solo iter() + fold
    Sin map, filter, find
    Sin mutabilidad
    */
pub fn first_negative(slice: &[i32]) -> Option<i32>
{
    slice.iter().fold(None,|acc:Option<i32>,&x|
        {
            acc.or_else(|| if x < 0 {Some(x)}else{None})
        })
}
pub fn second_even(slice: &[i32]) -> Option<i32>
{
    let (_,second) = slice.iter().fold((None,None),
     |(first,second),&x|
    {
        // si x % 2 == 0 && acc == 1 => Some(x)
        if x % 2 == 0
        {
            match (first,second) {
                (None,_)        => (Some(x),second),
                (Some(_),None)  => (first,Some(x)),
                _               => (first,second),
            }
        }
        else
        {
            (first,second)
        }
    },);
    second
}