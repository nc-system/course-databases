
## Diagrama Físico: normalización

    - La normalización como su nombre lo indica nos ayuda a dejar todo de una forma normal. Esto obedece a las 12 reglas de Codd y nos permiten separar componentes en la base de datos:

    Primera forma normal (1FN): Atributos atómicos (Sin campos repetidos)
    Segunda forma normal (2FN): Cumple 1FN y cada campo de la tabla debe depender de una clave única.
    Tercera forma normal (3FN): Cumple 1FN y 2FN y los campos que NO son clave, NO deben tener dependencias.
    Cuarta forma normal (4FN): Cumple 1FN, 2FN, 3FN y los campos multivaluados se identifican por una clave única.



## Secuencia

    - Tabla original sin normalizacion


<img src="../img/normalization-table-original.png">

<br><br>

    1 Aplicamos primera forma normal (1FN) y queda asi:
<img src="../img/normalization-table-1fn.png">

<br><br>

    2 - Aplicamos primera forma normal (2FN) y queda asi:
<img src="../img/normalization-table-2fn.png">

<br><br>

    3 - Aplicamos primera forma normal (3FN) y queda asi:
<img src="../img/normalization-table-3fn.png">

<br><br>

    4 - Aplicamos primera forma normal (4FN) y queda asi:
<img src="../img/normalization-table-4fn.png">

<br><br>