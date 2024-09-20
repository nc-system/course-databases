
# INSTALL POSTGRES SQL

## INSTALL LINUX

### Debian

#### ubuntu Server 24

  1. Actualiza los repositorios y paquetes  

    sudo apt update && sudo apt upgrade -y

  2. Instala  PostgreSQL y el paquete contrib

    sudo apt install postgresql postgresql-contrib
  
  3. Verifica el estado del servicio de PostgreSQL

    sudo systemctl status postgresql

  4. Accede al shell de PostgreSQL

    sudo -i -u postgres
    
    psql

  5. Crea un nuevo usuario y una base de datos (Opcional)

    postgres@server:~$

      CREATE USER tu_usuario WITH PASSWORD 'tu_contraseña';
      CREATE DATABASE tu_base_de_datos;
      GRANT ALL PRIVILEGES ON DATABASE tu_base_de_datos TO tu_usuario;
  
6. Sal del shell de PostgreSQL

    postgres@server:~$ \q

    postgres@server:~$ exit

## INSTALL MAC

## INSTALL WINDOWS
