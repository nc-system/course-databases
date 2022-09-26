
MySQL Server – Ajustar la autenticación y los privilegios del usuario
Para las instalaciones recientes, querrá ejecutar el script de seguridad que viene incluido. Esto cambia algunas de las opciones predeterminadas menos seguras para cosas como inicios de sesión root remotos y usuarios de ejemplo. Para las versiones antiguas de MySQL, también deberá inicializar el directorio de datos manualmente, pero ahora esto se hace automáticamente.


Ejecute el script de seguridad:

sudo mysql_secure_installation

Note que, si bien estableció una contraseña para el usuario root de MySQL, este usuario no está configurado para autenticarse con una contraseña al conectarse al shell de MySQL.

Para los sistemas Ubuntu que estén usando MySQL 5.7 (y las versiones posteriores), el usuario root de MySQL está configurado, de forma predeterminada, para autenticarse usando el plugin auth_socket en vez de una contraseña. En muchos casos, esto permite que la seguridad y usabilidad sea mayor pero también puede complicar las cosas cuando deba permitir que un programa externo (tal como phpMyAdmin) tenga acceso al usuario.

Deberá cambiar su método de autenticación de auth_socket a mysql_native_password para usar una contraseña para conectarse a MySQL como root. Para hacerlo, abra la indicación de MySQL desde su terminal:

sudo mysql

Posteriormente, consulte cuál método de autenticación usa cada una de sus cuentas de usuario de MySQL usando el siguiente comando:

SELECT user,authentication_string,plugin,host FROM mysql.user;

Output

+------------------+-------------------------------------------+-----------------------+-----------+
| user             | authentication_string                     | plugin                | host      |
+------------------+-------------------------------------------+-----------------------+-----------+
| root             |                                           | auth_socket           | localhost |
| mysql.session    | *THISISNOTAVALIDPASSWORDTHATCANBEUSEDHERE | mysql_native_password | localhost |
| mysql.sys        | *THISISNOTAVALIDPASSWORDTHATCANBEUSEDHERE | mysql_native_password | localhost |
| debian-sys-maint | *CC744277A401A7D25BE1CA89AFF17BF607F876FF | mysql_native_password | localhost |
+------------------+-------------------------------------------+-----------------------+-----------+
4 rows in set (0.00 sec)
En este ejemplo, puede ver que el usuario root verdaderamente se autentica usando el plugin auth_socket. Para configurar la cuenta root para autenticarse usando una contraseña, ejecute el siguiente comando ALTER USER. Asegúrese de cambiar password (contraseña) a una contraseña segura de su elección y sepa que este comando cambiará la contraseña de root que estableció en el Paso 2:

ALTER USER 'root'@'localhost' IDENTIFIED WITH mysql_native_password BY 'password';

Luego, ejecute FLUSH PRIVILEGES (purgar privilegios), lo que le dice al servidor que vuelva a cargar las tablas grant e implemente sus nuevos cambios:

FLUSH PRIVILEGES;

Vuelva a verificar los métodos de autenticación que usa cada uno de sus usuarios para confirmar que root ya no se autentica usando el plugin auth_socket:

SELECT user,authentication_string,plugin,host FROM mysql.user;

Output

+------------------+-------------------------------------------+-----------------------+-----------+
| user             | authentication_string                     | plugin                | host      |
+------------------+-------------------------------------------+-----------------------+-----------+
| root             | *3636DACC8616D997782ADD0839F92C1571D6D78F | mysql_native_password | localhost |
| mysql.session    | *THISISNOTAVALIDPASSWORDTHATCANBEUSEDHERE | mysql_native_password | localhost |
| mysql.sys        | *THISISNOTAVALIDPASSWORDTHATCANBEUSEDHERE | mysql_native_password | localhost |
| debian-sys-maint | *CC744277A401A7D25BE1CA89AFF17BF607F876FF | mysql_native_password | localhost |
+------------------+-------------------------------------------+-----------------------+-----------+
4 rows in set (0.00 sec)
En este resultado de ejemplo, puede ver que, ahora, el usuario root de MySQL se autentica usando una contraseña. Una vez que confirme esto en su propio servidor, puede salir del shell de MySQL:

exit

Alternativamente, para otras personas puede adaptarse mejor a su flujo de trabajo si se conectan a MySQL con un usuario dedicado. Para crear tal usuario, vuelva a abrir el shell de MySQL nuevamente:

sudo mysql
Nota: Si tiene la autenticación de contraseña habilitada para root según se describió en los párrafos de arriba, deberá usar un comando diferente para acceder al shell de MySQL. Lo que se indica a continuación ejecutará su cliente MySQL con privilegios de usuario regular, y solamente tendrá privilegios de administrador dentro de la base de datos una vez que haga la autenticación:

mysql -u root -p

Desde ese punto, cree un nuevo usuario y use una contraseña sólida:

CREATE USER 'sammy'@'localhost' IDENTIFIED BY 'password';

Luego, dele a su nuevo usuario los privilegios adecuados. Por ejemplo, puede concederle al usuario privilegios a todas las tablas dentro de la base de datos, así como autoridad para agregar, cambiar y eliminar privilegios de usuario, mediante este comando:

GRANT ALL PRIVILEGES ON *.* TO 'sammy'@'localhost' WITH GRANT OPTION;

Note que, en este punto, no necesita volver a ejecutar el comando FLUSH PRIVILEGES. Solamente necesita este comando al modificar las tablas grant utilizando declaraciones como INSERT, UPDATE o DELETE. Dado a que creó un usuario nuevo en vez de modificar uno existente, no es necesario que use FLUSH PRIVILEGES aquí.

Después de esto, salga del Shell de MySQL:

exit

Por último, vamos a probar la instalación de MySQL.

FLUSH PRIVILEGES;