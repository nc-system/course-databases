# TABLES
  
-

## View tables

  mydb=# \dt

## Create tables

   mydb=> CREATE TABLE cities (
    name  varchar(80),
    location  point
   );

## Edit tables

## Delete tables

  mydb=# DROP TABLE tablename;

## Show table structure

  mydb=# \d name_table

## Add column to table

  mydb=# ALTER TABLE nametable ADD COLUMN name_column INTEGER;

