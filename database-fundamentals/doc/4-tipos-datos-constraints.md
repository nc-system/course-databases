
## Diagrama Físico: tipos de datos y constraints

    - Para llevar a la práctica un diagrama debemos ir más allá y darle detalle con parámetros como:

        Tipos de dato:

        Texto: CHAR(n), VARCHAR(n), TEXT
        Números: INTEGER, BIGINT, SMALLINT, DECIMAL(n,s), NUMERIC(n,s)
        Fecha/hora: DATE, TIME, DATETIME, TIMESTAMP
        Lógicos: BOOLEAN
        Constraints (Restricciones)

        NOT NULL: Se asegura que la columna no tenga valores nulos
        UNIQUE: Se asegura que cada valor en la columna no se repita
        PRIMARY KEY: Es una combinación de NOT NULL y UNIQUE
        FOREIGN KEY: Identifica de manera única una tupla en otra tabla
        CHECK: Se asegura que el valor en la columna cumpla una condición dada
        DEFAULT: Coloca un valor por defecto cuando no hay un valor especificado
        INDEX: Se crea por columna para permitir búsquedas más rápidas



### Tipos de datos

    Tipos de datos en BB.DD

    Texto:

    CHAR(n)
    La diferencia entre CHAR y VARCHAR es que originalmente CHAR guardaba el espacio de caracteres que le asignaras, no importaba si lo utilizabas o no (ocupando siempre la misma memoria). En cambio, VARCHAR, aunque también pedía un espacio mínimo, dependiendo si lo utilizabas o no, aumentaba o disminuía (ocupando solo la memoria que necesitas)

    VARCHAR(n) En el paréntesis se escribe el número de caracteres a utilizar, en caso de que no se tenga claro, se puede dejar por defecto, permitiendo una máx. de 255 caracteres.

    TEXT También se utiliza para texto, pero una gran cantidad de caracteres.

    Numérico:
    Nos ayudan a guardar números, duhh. También se podría pensar que los números podrían ser almacenados en datos de tipo texto, pero no… No almenos si quieres operar (Sumarlos, multiplicar, restar…)

    INTENGER: Para guardar números enteros.
    (SUBTIPOS)

    BIGINT (Tiene una función similar a la INTERGER, pero la utilizamos cuando sabemos que necesitamos trabajar con números muy grandes)
    
    SMALLINT (Tiene una función similar a la INTERGER, pero la utilizamos cuando sabemos que necesitamos trabajar con números “pequeños”)
    En este enlace de Microsoft publica cuando utilizar cada uno en SQL Microsoft
    
    DECIMAL (n, s) Es un tipo de dato al cual se le pueden pasar dos valores, siendo “n” el número con decimales, y “s” la cantidad de decimales que queremos que almacene
   
   


    Fecha y hora:
    Este tipo de datos son muy interesantes, ya que nos permiten guardar información de cada evento de tiempo.

    DATE Año, mes y día.

    TIME La hora del día de las 24h.

    DATETIME Obtiene los dos anteriores




    Lógico:

    BOOLEAN Solo puede guardar dos valores (BINARIO) 1 o 0, vivo o muerto, verdad o falso, encendido o apagado.
    Otro parámetro sumamente importante son los constraints o restricciones, que son otro tipo de regla para la información que van a guardar nuestros atributos.

    NOT NULL: Exige que el campo no se quede vacío, es decir que no tenga valores nulos.
    UNIQUE: Se asegura que cada valor en la columna no se repita.
    PRIMARY KEY: Es una combinación entre NOT NULL & UNIQUE.
    FOREING KEY: Nos permite relacionar las tablas, es el identificador de una entidad en otra. (foránea= de afuera, externa)
    CHECK: Se asegura que se guarden registros que cumplan una condición.
    DEFAULT: Cuando no se llena el campo, se asigna un valor por defecto (no), y cuando queremos que tenga otro valor, utilizamos esta constraint.
    INDEX: Se crea un sistema de búsqueda más eficiente, el problema es que cada vez que se agregan datos, tiene que re indexar todos los registros, pero es muy útil cuando se necesita consultar muchas veces la BB.DD