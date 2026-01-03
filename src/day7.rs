use std::result;

// ejemplo
struct PrefixSums(Vec<i32>);
impl FromIterator<i32> for PrefixSums {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut acc = 0;
        let mut result = vec::new();

        for x in iter{
            acc += x;
            result.push(acc);
        }
        PrefixSums(result)
    }
}
// uso
// let ps: PrefixSums = vec![1,2,3].into_iter().collect();

// no devuelves Vec , devuelves capacidad de composicion
fn evens<I: IntoIterator<Item = i32>>(iter: I) -> impl Iterator<Item = i32>
{
    iter.into_iter().filter(|x| x % 2 == 0)
}
// --------------------------------------------------------------------- //

// Ejercicio 1
// implementa un iterador que produzca : 0,1,1,2,3,5,8,...
// regla : sin vectores, solo estado , next() puro
struct Fibonacci;



// ejercicio 2
// funcion que retome solo los elementos en posiciones pares
/* fn every_other<I>(iter: I) -> impl Iterator<Item = I::Item>
where
    I: Iterator,*/
