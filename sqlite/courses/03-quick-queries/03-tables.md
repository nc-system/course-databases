
# TABLES


## Create tables
  
    sqlite> CREATE TABLE users (id_user INTEGER PRIMARY KEY, name TEXT, last_name TEXT, email TEXT, phone TEXT, status INTEGER);

    sqlite> CREATE TABLE IF NOT EXISTS users (id_users INTEGER PRIMARY KEY, name TEXT, last_name TEXT, email TEXT, phone TEXT, status INTEGER);


## Delete tables