use std::iter::FusedIterator;

// Implementa un iterador infinito
/*
Reglas: 
    No usar <std::iter::repeat>
    Estado minimo
    API Limpia
*/
struct Repeater
{
    num: i32,
}
impl Iterator for Repeater
{
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item>
    {
        Some(self.num)
    }
}
impl FusedIterator for Repeater{}

pub fn repeat_value(x:i32)-> impl Iterator<Item=i32>
where 
    {
        Repeater{num : x}
    }

/*
    Ejercicio 2 
        el iterador decide cuando cortar
        implementa <FusedIterator>
        Contrato : 
        - Produce sumas acumuladas
        - Se detiene cuando la suma supera el límite
        - El valor que supera el límite SÍ se emite
        - Después de devolver None, siempre devuelve None (Fused)
*/
struct Sum<I>
{
    iter: I,
    acc: i32,
    limit: i32,
}
impl<I> Iterator for Sum<I>
where 
    I: Iterator<Item = i32>,
{
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item>
    {
        if self.acc > self.limit
        {
            return None;
        }
        let next_val = self.iter.next()?;
        self.acc += next_val;
        Some(self.acc)
    }
}
impl<I> FusedIterator for Sum<I>
where
    I: FusedIterator<Item= i32>,{}
pub fn running_sum_until<I>(iter: I,limit: i32) 
        -> impl Iterator<Item = i32> + FusedIterator
where
    I: FusedIterator<Item = i32>,
{
    Sum{iter,limit,acc:0}
}