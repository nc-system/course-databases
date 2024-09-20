
# Para activar conexiones remotas a PostgreSQL, sigue estos pasos:

## 1. Editar el archivo postgresql.conf:

    - Abre el archivo postgresql.conf con un editor de texto. Este archivo generalmente se encuentra en el directorio /etc/postgresql/<versión>/main/.

    - Busca la línea 
    
        #listen_addresses = 'localhost' 
        
            y cámbiala por 
        
        listen_addresses = '*' 
        
    para permitir conexiones desde cualquier IP1.


## 2. Configurar el archivo pg_hba.conf:
    
    - Abre el archivo pg_hba.conf, que también se encuentra en el directorio /etc/postgresql/<versión>/main/.

    - Añade una línea como esta para permitir conexiones IPv4 desde cualquier IP:

        host    all             all             0.0.0.0/0               md5


## 3. Abrir el puerto en el firewall:
    
    - Asegúrate de que el puerto 5432 (el puerto por defecto de PostgreSQL) esté abierto en el firewall. Puedes hacerlo con el siguiente comando:

        $ sudo ufw allow 5432/tcp


## 4. Reiniciar el servicio de PostgreSQL:

    - Aplica los cambios reiniciando el servicio de PostgreSQL:

        $ sudo service postgresql restart

    - Después de completar estos pasos, deberías poder conectarte a tu servidor PostgreSQL desde una máquina remota usando un cliente PostgreSQL12.
