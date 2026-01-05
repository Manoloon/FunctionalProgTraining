// ejemplo
struct PrefixSums(Vec<i32>);
impl FromIterator<i32> for PrefixSums {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut acc = 0;
        let mut result = Vec::new();

        for x in iter {
            acc += x;
            result.push(acc);
        }
        PrefixSums(result)
    }
}
// uso
// let ps: PrefixSums = vec![1,2,3].into_iter().collect();

// no devuelves Vec , devuelves capacidad de composicion
fn evens<I: IntoIterator<Item = i32>>(iter: I) -> impl Iterator<Item = i32> {
    iter.into_iter().filter(|x| x % 2 == 0)
}
// --------------------------------------------------------------------- //

// Ejercicio 1
// implementa un iterador que produzca : 0,1,1,2,3,5,8,...
// regla : sin vectores, solo estado , next() puro
pub struct Fibonacci {
    current: i32,
    next: i32,
    limit: i32,
}
impl Fibonacci {
    pub fn new(limit: i32) -> Self {
        Fibonacci {
            current: 0,
            next: 1,
            limit,
        }
    }
}
impl Iterator for Fibonacci {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.limit {
            None
        } else {
            let val = self.current;
            let new_next = self.current + self.next;
            self.current = self.next;
            self.next = new_next;
            Some(val)
        }
    }
}

pub struct RunningSum<I> {
    iter: I,  // iterador interno
    acc: i32, // estado acumulado
}
// constructor funcional
pub fn running_sum<I>(iter: I) -> RunningSum<I>
where
    I: Iterator<Item = i32>,
{
    RunningSum { iter, acc: 0 }
}
impl<I> Iterator for RunningSum<I>
where
    I: Iterator<Item = i32>,
{
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let next_val = self.iter.next()?; // corta la implementacion 
        let new_val = self.acc + next_val;
        self.acc = new_val;
        Some(new_val)
    }
}

// Implementa un iterador genérico que produzca el producto acumulado.
pub struct RunningProduct<I> {
    iter: I,
    acc: i32,
}
pub fn running_product<I>(iter: I) -> RunningProduct<I>
where
    I: Iterator<Item = i32>,
{
    RunningProduct { iter, acc: 1 }
}
impl<I> Iterator for RunningProduct<I>
where
    I: Iterator<Item = i32>,
{
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let next_val = self.iter.next()?;
        let new_val = self.acc * next_val;
        self.acc = new_val;
        Some(new_val)
    }
}
// Transformadores con estado opcional 
/*
Reimplementa RunningMax, pero ahora:
    Debe empezar vacío
    El primer elemento define el estado
    No puede asumir valores iniciales
*/
pub struct RunningMax<I>
{
    curr_max : Option<i32>,
    iter : I,
}
pub fn running_max<I>(iter: I) -> RunningMax<I>
where 
    I: Iterator<Item = i32>,
    {
        RunningMax { curr_max: None, iter }
    }
impl<I> Iterator for RunningMax<I>
where 
    I: Iterator<Item = i32>,
{
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item>
    {
        let next_val = self.iter.next()?;
        let new_max = match self.curr_max
        {
            Some(m) => m.max(next_val),
            None => next_val,
        };
        self.curr_max = Some(new_max);
        Some(new_max)
    }
}

// Transformadores que filtran
/*
    Crea un iterador que ignore valores hasta que se cumpla una condición.
    * El predicado se evalúa una sola vez
    * Luego pasa todo sin filtrar
    drop_until(iter, |x| x > 0)
*/
pub struct DropUntil<I,P>
{
    iter: I,
    pred: P,
    started: bool,
}
pub fn drop_until<I,P>(iter: I,pred: P) -> DropUntil<I,P>
where
    I: Iterator<Item = i32>,
    P: Fn(i32) -> bool,
    {
        DropUntil { iter, pred, started: false }
    }
impl <I,P> Iterator for DropUntil<I,P> 
where 
    I: Iterator<Item = i32>,
    P: Fn(i32) -> bool,
    {
        type Item = i32;
        fn next(&mut self) ->Option<Self::Item>
        {
            while let Some(x) = self.iter.next()
            {
                if self.started
                {
                    return Some(x)
                }
                if (self.pred)(x)
                {
                    self.started = true;
                    return Some(x)
                }    
            }
            None
        }
    }

// Iterador con estado compuesto
/*
    Ventanas deslizantes
    Manejo Correcto del primer elemento
*/
pub struct PairWiseSum<I>
{
    iter: I,
    prev: Option<i32>,
}
pub fn pair_wise_sum<I>(iter: I)-> PairWiseSum<I>
where 
    I: Iterator<Item = i32>,
    {
        PairWiseSum { iter, prev: None}
    }
impl<I> Iterator for PairWiseSum<I>
where 
    I: Iterator<Item=i32>,
    {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item>
        {
            let next_val = self.iter.next()?;
            if let Some(prev) = self.prev
            {
                self.prev = Some(next_val);
                Some(prev + next_val)
            }
            else
            {
                self.prev = Some(next_val);
                self.next()
            }
        }
    }

// API limpia
/*
Requisitos:
    El tipo concreto NO debe ser visible
    Los tests deben seguir funcionando
Objetivo:
    Entender cuándo ocultar tipos
    Diseñar APIs estables
    ejemplo : pub fn foo<I>(iter: I) -> impl Iterator<Item = i32>
*/
pub fn pair_wise_hide<I>(iter: I) -> impl Iterator<Item = i32>
where 
    I: Iterator<Item = i32>,
    {
        PairWiseSum{iter,prev: None}
    }