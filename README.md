# rust-dsa

Data Structures &amp; Algorithms implemented in Rust

<p align="center">
  <a href="https://www.geeksforgeeks.org/learn-data-structures-and-algorithms-dsa-tutorial/?ref=ds_lp" target="blank"><img src="https://github.com/mnsh4/rust-dsa/blob/main/docs/DSA-GeeksforGeeks.png" width="" alt="DSA" /></a>
</p>

***¿Qué es la estructura de datos?***

Una estructura de datos se define como una forma particular de almacenar y organizar datos en nuestros dispositivos para utilizarlos de manera eficiente y efectiva. La idea principal detrás del uso de estructuras de datos es minimizar las complejidades de tiempo y espacio. Una estructura de datos eficiente requiere un espacio de memoria mínimo y un tiempo mínimo para ejecutar los datos.

***¿Qué es el algoritmo?***

Un algoritmo se define como un proceso o conjunto de instrucciones bien definidas que normalmente se utilizan para resolver un grupo particular de problemas o realizar un tipo específico de cálculo. Para explicarlo en términos más simples, es un conjunto de operaciones realizadas paso a paso para ejecutar una tarea.

***¿Cómo empezar a aprender DSA?***

Lo primero y más importante es dividir el procedimiento total en pequeñas partes que deben realizarse de forma secuencial. El proceso completo para aprender DSA desde cero se puede dividir en 4 partes:

1. Aprenda sobre las complejidades del tiempo y el espacio

2. Aprenda los conceptos básicos de las estructuras de datos individuales

3. Aprenda los conceptos básicos de los algoritmos.

4. Problemas de práctica en DSA 

***Aprenda sobre las complejidades***

El motivo principal para utilizar DSA es ´resolver un problema de forma eficaz y eficiente´. ¿Cómo puedes decidir si un programa escrito por ti es eficiente o no? Esto se mide por las complejidades. La complejidad es de dos tipos:

1. Complejidad del tiempo: la complejidad del tiempo se utiliza para medir la cantidad de tiempo necesaria para ejecutar el código.

Comprender la complejidad del tiempo con ejemplos simples:

Muchos estudiantes se confunden al comprender el concepto de complejidad del tiempo, pero en este artículo lo explicaremos con un ejemplo muy simple.

Imagine una clase de 100 estudiantes en la que le dio su bolígrafo a una persona. Tienes que encontrar ese bolígrafo sin saber a quién se lo regalaste. 

A continuación se muestran algunas formas de encontrar el bolígrafo y cuál es el orden O.

**O(n 2 ):** Vas y le preguntas a la primera persona de la clase si tiene el bolígrafo. Además, le preguntas a esta persona sobre las otras 99 personas en el salón de clases si tienen ese bolígrafo, etc. Esto es lo que llamamos O(n 2 ). 

**O(n):** Ir y preguntar a cada estudiante individualmente es O(N). 

**O(log n):** Ahora divido la clase en dos grupos y luego pregunto: “¿Está en el lado izquierdo o derecho del salón de clases?” Luego tomo ese grupo y lo divido en dos y vuelvo a preguntar, y así sucesivamente. Repita el proceso hasta que le quede un estudiante que tenga su bolígrafo. Esto es lo que quieres decir con O (log n). 
Puede que necesite hacer:

El O(n 2 ) busca si solo un estudiante sabe en qué estudiante está escondido el bolígrafo . 

El O(n) si un estudiante tenía el bolígrafo y solo él lo sabía . 

La búsqueda O(log n) si todos los estudiantes lo supieran , pero solo me diría si adiviné el lado correcto. 

2. Complejidad espacial: la complejidad espacial significa la cantidad de espacio necesaria para ejecutar con éxito las funcionalidades del código. 

Ambas complejidades anteriores se miden con respecto a los parámetros de entrada. Pero aquí surge un problema. El tiempo necesario para ejecutar un código depende de varios factores, como por ejemplo: 

El número de operaciones realizadas en el programa, la velocidad del dispositivo, y también la velocidad de transferencia de datos si se ejecuta en una plataforma en línea. 

Entonces, ¿cómo podemos determinar cuál es eficiente? La respuesta es el uso de notación asintótica. 

Las siguientes tres notaciones asintóticas se utilizan principalmente para representar la complejidad temporal de los algoritmos:

Big-O Notation (Ο): la Big-O Notation describe específicamente el peor de los casos.
Omega Notation (Ω): la notación Omega (Ω) describe específicamente el mejor de los casos.
Theta Notation (θ): esta notación representa la complejidad promedio de un algoritmo.

La notación más utilizada en el análisis de un código es la notación O grande , que proporciona un límite superior del tiempo de ejecución del código (o la cantidad de memoria utilizada en términos de tamaño de entrada)

<p align="center">
  <a href="https://www.freecodecamp.org/news/all-you-need-to-know-about-big-o-notation-to-crack-your-next-coding-interview-9d575e7eec4/" target="blank"><img src="https://cdn-media-1.freecodecamp.org/images/1*KfZYFUT2OKfjekJlCeYvuQ.jpeg" width="" alt="DSA" /></a>
</p>