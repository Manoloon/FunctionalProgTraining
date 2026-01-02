
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