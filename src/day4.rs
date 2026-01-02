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