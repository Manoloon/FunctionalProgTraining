
// dia 1
pub fn double(n:i32) -> i32{ n * n }
pub fn is_even(n:i32) -> bool{ n % 2 == 0}
pub fn clamp(n:i32,min:i32,max:i32) -> i32
{
    match n
    {
       n if n < min => min,
       n if n > max => max,
       n => n,
    }
}
// day 2
pub fn factorial(n:u32) -> u32
{
    // caso base
    if n < 1 {return 1;}
    // paso recursivo 
    n * factorial(n-1)
    // se reduce en cada llamada : n
}
pub fn sum(slice: &[i32]) -> i32
{
    if slice.is_empty()     // Caso base:
    {
        0
    }
    else 
    {
        slice[0] + sum(&slice[1..])     // Paso recursivo:
    }
    // Qué se reduce en cada llamada: la longitud de slice en 1 
}

pub fn length(s: &str) -> usize
{
    fn helper(mut chars: std::str::Chars) -> usize{ // chars no es un array sino es un conjunto de UTF8 
        match chars.next() {
            None => 0, // caso base
            Some(_) => 1 + helper(chars), // paso recursivo
        }
    }
    helper(s.chars())
}
pub fn all_positive(slice: &[i32]) -> bool
{
   if slice.is_empty()
   {
    true // caso base
   }
   else
   {
    slice[0] > 0 && all_positive(&slice[1..]) 
   }
    // match slice {
    //     [] => true,
    //     [head, tail @ ..] => *head > 0 && all_positive(tail),
    // }
    // functional
    // slice.iter().all(|x| *x > 0);
}

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

fn main(){}