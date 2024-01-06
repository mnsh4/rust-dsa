# Array Data Structure

Es una estructura de datos lineal. Una matriz es una colección de tipos de datos homogéneos donde a los elementos se les asigna memoria contigua. Debido a la asignación contigua de memoria, se puede acceder a cualquier elemento de una matriz en tiempo constante. Cada elemento de la matriz tiene un número de índice correspondiente. 

La idea es almacenar varios artículos del mismo tipo juntos. Esto hace que sea más fácil calcular la posición de cada elemento simplemente añadiendo un desplazamiento a un valor base, es decir, la ubicación de memoria del primer elemento de la matriz (generalmente indicado por el nombre de la matriz).

<p align="center">
  <a href="https://www.geeksforgeeks.org/learn-data-structures-and-algorithms-dsa-tutorial/?ref=ds_lp" target="blank"><img src="https://media.geeksforgeeks.org/wp-content/cdn-uploads/20230726162247/Array-data-structure.png" width="" alt="Array Data Structure" /></a>
</p>

Podemos acceder directamente a un elemento de matriz utilizando su valor de índice

***Terminologías básicas de matriz:***

**Array index**: en un array, los elementos se identifican por sus índices. El índice del array comienza desde 0.

**Array element**: los elementos son elementos almacenados en un array y se puede acceder a ellos mediante su índice.

**Array Length**: la longitud de un array está determinada por la cantidad de elementos que puede contener. 

```rust
fn main() {
    let arr = vec!["Argentina", "Japon", "Paraguay"];
    println!("{:?}", arr);

    println!("{}", arr[0]);
    println!("{}", arr[1]);
    println!("{}", arr[2]);
  

    for (i, &v) in arr.iter().enumerate() {
        println!("Array Index: {} - Array Element: {}", i, v);
    }
    println!("Array Length: {}", arr.len());
}
```

***¿Por qué se necesitan estructuras de datos de array?***

Supongamos que hay una clase de cinco estudiantes y si tenemos que mantener registros de sus calificaciones en los exámenes, podemos hacerlo declarando cinco variables individuales y manteniendo un registro de los registros, pero ¿qué pasa si el número de estudiantes se vuelve muy grande? difícil manipular y mantener los datos.

Lo que significa es que podemos usar variables normales (v1, v2, v3, ...) cuando tenemos una pequeña cantidad de objetos. Pero si queremos almacenar una gran cantidad de instancias, resulta difícil administrarlas con variables normales. La idea de una matriz es representar muchas instancias en una variable

<p align="center">
  <a href="https://www.geeksforgeeks.org/learn-data-structures-and-algorithms-dsa-tutorial/?ref=ds_lp" target="blank"><img src="https://media.geeksforgeeks.org/wp-content/uploads/20220728175153/Needforarray-660x379.png" width="" alt="Array Data Structure" /></a>
</p>