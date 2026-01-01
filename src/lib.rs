use std::result;


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

// Day 4 Buscar con Option
pub fn first_even(slice: &[i32]) -> Option<i32>
{
    slice.iter().find(|&&x| x % 2 == 0).copied()
}
// La string debe ser un número
// Debe ser mayor que 0
// Cada error debe ser explícito
#[derive(Debug,PartialEq)]
pub enum ParseErrors {
        NotANumber,
        NotPositive,
}
pub fn parse_positive(s: &str) -> Result<i32, ParseErrors>
{
    s.parse::<i32>().map_err(|_| ParseErrors::NotANumber)
                    .and_then(|n| if n > 0 
                        {
                            Ok(n)
                        }
                        else
                        {
                            Err(ParseErrors::NotPositive)
                        })
}
// Reutiliza first_even
pub fn double_first_even(slice: &[i32]) -> Option<i32>
{
    // map → transforma el valor
    // and_then → transforma y puede fallar 
    first_even(slice).map(|n| n*2)
}

pub fn first_positive_even(slice: &[i32]) -> Option<i32>
{
    slice.iter().filter(|&&n| n > 0 )
                .find(|&&n| n % 2 == 0).copied()
}

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

// Day 6
pub fn product(slice: &[i32]) -> Option<i32>
{
    slice.iter().copied().reduce(|a,b| a * b)
}
// use scan
// retoma el valor anterior maximo y compara el proximo
pub fn running_max(slice: &[i32]) -> Vec<i32>
{
    slice.iter().scan(None,|state: &mut Option<i32>,&x| 
        {
            let new_max = match *state
            {
              Some(current) => current.max(x), // el valor max entre el anterior y el current
              None               => x, // si x es nada , devuelvo None y termino.
            };

            *state = Some(new_max);
            Some(new_max)
        }).collect()
}
// retoma el primer valor que sea menor que el anterior
// scan + estado
// acumulador ≠ resultado
pub fn first_drop(slice: &[i32]) -> Option<i32>
{
    slice.iter().scan(None,|prev,&x|
        {
            let result = match *prev
            {
                Some(r)  if x < r   => Some(x), // el valor anterior cotejado con el current
                _                       => None,
            };
            *prev = Some(x);
            Some(result)
        }).flatten()
        .next()
}
fn main(){}
