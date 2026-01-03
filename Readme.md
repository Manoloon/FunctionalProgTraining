### Programacion Funcional en Rust
# Conceptos claves :
1) Funciones puras
    * La salida depende unicamente de la entrada.
    * No cambia el estado global ni hace I/O inesperado.
2) Inmutabilidad
    * Variables no se reasignan
3) Composicion de funciones
    * Combina funciones pequeñas para crear funciones complejas.
4) Recursion en lugar de bucles
    * Los bucles mutables se reemplazan con llamadas recursivas o iteradores.
5) Tipos algebraicos
    * <enum> y <Option>, <Result> en Rust permiten modelar valores opcionales y errores de manera segura.
6) Funciones de orden superior
    * Funciones que reciben o devuelven otras funciones.
## Dia 1
    # funciones puras
    * "Una funcion es una transformacion de valores, no una secuencia de pasos."
## Dia 2
    # Recursion funcional
    * Caso Base, Paso recursivo y Que se reduce en cada llamada .
    * Si estas escribiendo indices (i,j), probablemente estas luchando contra Rust.
    * Piensa en : <map>, <filter>, <fold>, <all>, <any>
## Dia 3
    # Iteradores como recursion encapsulada
    * " Transformo cada elemento y recolecto el resultado" 
    * Reemplazar recursion explicita por iteradores
    * Uso de <map>, <filter>, <fold>
    * Correspondencia mental : 
    | Recursión         | `fold`              |
    | ----------------- | ------------------- |
    | caso base         | valor inicial       |
    | acumulador        | `acc`               |
    | llamada recursiva | siguiente iteración |
    * Describir el problema como : Tomar -> Filtrar -> Transformar -> Reducir => iter()
## Dia 4
    # <Option>, <Result> y diseño funcional con tipos
    * Eliminar bool ambiguos.
    * Transformar un <Option> (sin <if>) : Si hay valor -> se transforma, si no -> se propaga <None>.
    * <Result> exito o error con informacion, no hay estados invalidos ocultos.
    * " Un Valor que puede seguir fluyendo... o detener todo."
    * <and_then> Si soy <Ok>, aplica la funcion, si soy <err>, no hagas nada.
    * Diferencia entre <map> y <and_then> : Usa <map> cuando no puedes fallar, usa          <and_then> cuando la funcion tambien devuelve <Result>.
    * El operador <?> es azucar sintactico funcional, no magia.
    ** Regla de Oro : Nunca llames a <unwrap> en codigo funcional real.
## Dia 5
    # <fold> : recursion explicita y acumuladores
    * Idea central : toda recursion funcional tiene tres partes:
        1) Caso base
        2) Valor Actual
        3) Acumulador
        * <fold> encapsula esto de forma segura e inmutable.
        * <fold> evita stack overflow, es mas expresivo y es composable.
        * <if> es aceptable si : 
            * no controla flujo global
            * no introduce estado mutable
            * solo decide el proximo acumulador
## Dia 6
    # <reduce>, <scan> y pensamiento algebraico
    * <reduce> -> el primer elemento es el caso base
    * <fold> -> tu eliges el caso base
    * Regla de oro : usa <reduce> solo si : 
        * La operacion es asociativa
        * no necesitas estado adicional
        * el tipo no cambia
        -- Casos uso : suma, maximo, minimo, concatenacion   
## Dia 7
# iteradores avanzados y composicion 
Un iterador es una maquina de estados que produce valores bajo demanda
```
trait iterator
{
    type Item;
    fn next(&mut self)->Option<Self::Item>;
}
```
** Cada llamada a next():
* observa el estado
* decide si termina
* produce un valor
* actualiza el estado
##