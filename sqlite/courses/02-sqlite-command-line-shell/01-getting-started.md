
# GETTING STARTED

    - The SQLite project provides a simple command-line program named sqlite3 (or sqlite3.exe on Windows) that allows the user to manually enter and execute SQL statements against an SQLite database or against a ZIP archive. This document provides a brief introduction on how to use the sqlite3 program.

    - Start the sqlite3 program by typing "sqlite3" at the command prompt, optionally followed by the name of the file that holds the SQLite database (or ZIP archive). If the named file does not exist, a new database file with the given name will be created automatically. If no database file is specified on the command-line, a temporary database is created and automatically deleted when the "sqlite3" program exits.

    - On startup, the sqlite3 program will show a brief banner message then prompt you to enter SQL. Type in SQL statements (terminated by a semicolon), press "Enter" and the SQL will be executed.

    - For example, to create a new SQLite database named "ex1" with a single table named "tbl1", you might do this:

        $ sqlite3 ex1
        SQLite version 3.36.0 2021-06-18 18:36:39
        Enter ".help" for usage hints.
        
        sqlite> create table tbl1(one text, two int);
        sqlite> insert into tbl1 values('hello!',10);
        sqlite> insert into tbl1 values('goodbye', 20);
        sqlite> select * from tbl1;
        hello!|10
        goodbye|20
        sqlite>

    - Terminate the sqlite3 program by typing your system End-Of-File character (usually a Control-D). Use the interrupt character (usually a Control-C) to stop a long-running SQL statement.

    - Make sure you type a semicolon at the end of each SQL command! The sqlite3 program looks for a semicolon to know when your SQL command is complete. If you omit the semicolon, sqlite3 will give you a continuation prompt and wait for you to enter more text to complete the SQL command. This feature allows you to enter SQL commands that span multiple lines. 
    
    For example:

        sqlite> CREATE TABLE tbl2 (
        ...>   f1 varchar(30) primary key,
        ...>   f2 text,
        ...>   f3 real
        ...> );
        sqlite>