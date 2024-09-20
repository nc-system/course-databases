# COMMON COMMANDS

-

## Access to shell PostgreSQL

  $ sudo -i -u postgres

    - Access how root
    postgres@server:~$ psql

    or 

    - Access to database
    postgres@server:~$ psql mydb

## Exit from shell PostgreSQL

  postgres@server:~$ \q

  or

  postgres=# \q

  or

  mydb=> \q

## Help commands - Shows all commands

  postgres=# \h
  
  or

  mydb=> \h

## Show Version PostgreSQL

  postgres=# SELECT version();

  or

  mydb=> SELECT version();
