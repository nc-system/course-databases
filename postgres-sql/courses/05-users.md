# USERS

## Create users

  $ - CREATE USER tu_usuario WITH PASSWORD 'tu_contraseña';

## Edit users

## Delete users

## Add privileges

- Agregar privilegios a un usuario sobre un a base de datos

    postgres@server:~$

      GRANT ALL PRIVILEGES ON DATABASE tu_base_de_datos TO tu_usuario;

## Change user to superuser

- Para modificar un usuario existente en PostgreSQL y convertirlo en superusuario, puedes usar el comando ALTER ROLE.
  Aquí tienes los pasos:

  1. Conéctate a PostgreSQL:

    psql -U tu_usuario -d tu_base_de_datos

  2. Ejecuta el comando ALTER ROLE:
  
    SQL

    ALTER ROLE nombre_usuario WITH SUPERUSER;

    Ejemplo

    ALTER ROLE miusuario WITH SUPERUSER;

    Este comando le otorga al usuario miusuario todos los privilegios de un super usuario
