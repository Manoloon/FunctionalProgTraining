/*
Condición:
No emitir nada hasta tener dos valores
Estado mínimo
Tipo oculto
Resultado : [5, 8, 10, 7] → [3, 2, -3]
*/
struct Differences<I>
{
    iter : I,
    prev : Option<i32>,
}
impl<I> Iterator for Differences<I>
where
    I: Iterator<Item = i32>,
    {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item>
        {
            let next_val = self.iter.next()?;
            if let Some(prev) = self.prev
            {
                self.prev = Some(next_val);
                Some(next_val - prev)
            }
            else
            {
                self.prev = Some(next_val);
                self.next()
            }
        }
    }
pub fn diff_hide<I>(iter:I)-> impl Iterator<Item=i32>
where 
    I: Iterator<Item = i32>,
    {
        Differences{iter,prev: None}
    }

/*
Implementa un iterador que: 
Emite valores solo mientras estén creciendo
ejemplo : [1, 2, 3, 2, 5] → [1, 2, 3]
*/
struct Emitter<I>
{
    iter : I,
    prev : Option<i32>,
}
impl<I> Iterator for Emitter<I>
where
    I: Iterator<Item = i32>,
    {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item>
        {
            let next_val = self.iter.next()?;
            if let Some(prev) = self.prev
            {
                if next_val > prev
                {
                    self.prev = Some(next_val);
                    Some(next_val)
                }
                else
                {
                    None
                }
            }
            else
            {
                self.prev = Some(next_val);
                Some(next_val)
            }
        }
    }
pub fn emit_until<I>(iter:I)-> impl Iterator<Item=i32>
where 
    I: Iterator<Item = i32>,
    {
        Emitter{iter,prev: None}
    }