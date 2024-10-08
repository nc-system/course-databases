
# Querying the database schema

    - The sqlite3 program provides several convenience commands that are useful for looking at the schema of the database. There is nothing that these commands do that cannot be done by some other means. These commands are provided purely as a shortcut.

    For example, to see a list of the tables in the database, you can enter ".tables".

        sqlite> .tables
        tbl1 tbl2
        sqlite>

    - The ".tables" command is similar to setting list mode then executing the following query:

        SELECT name FROM sqlite_schema
        WHERE type IN ('table','view') AND name NOT LIKE 'sqlite_%'
        ORDER BY 1

    - But the ".tables" command does more. It queries the sqlite_schema table for all attached databases, not just the primary database. And it arranges its output into neat columns.

    - The ".indexes" command works in a similar way to list all of the indexes. If the ".indexes" command is given an argument which is the name of a table, then it shows just indexes on that table.

    - The ".schema" command shows the complete schema for the database, or for a single table if an optional tablename argument is provided:

        sqlite> .schema
        create table tbl1(one varchar(10), two smallint)
        CREATE TABLE tbl2 (
        f1 varchar(30) primary key,
        f2 text,
        f3 real
        );
        sqlite> .schema tbl2
        CREATE TABLE tbl2 (
        f1 varchar(30) primary key,
        f2 text,
        f3 real
        );
        sqlite>

    - The ".schema" command is roughly the same as setting list mode, then entering the following query:

        SELECT sql FROM sqlite_schema
        ORDER BY tbl_name, type DESC, name
    
    - As with ".tables", the ".schema" command shows the schema for all attached databases. If you only want to see the schema for a single database (perhaps "main") then you can add an argument to ".schema" to restrict its output:

        sqlite> .schema main.*

    - The ".schema" command can be augmented with the "--indent" option, in which case it tries to reformat the various CREATE statements of the schema so that they are more easily readable by humans.

    - The ".databases" command shows a list of all databases open in the current connection. There will always be at least 2. The first one is "main", the original database opened. The second is "temp", the database used for temporary tables. There may be additional databases listed for databases attached using the ATTACH statement. The first output column is the name the database is attached with, and the second result column is the filename of the external file. There may be a third result column which will be either "'r/o'" or "'r/w'" depending on whether the database file is read-only or read-write. And there might be a fourth result column showing the result of sqlite3_txn_state() for that database file.

        sqlite> .databases

    - The ".fullschema" dot-command works like the ".schema" command in that it displays the entire database schema. But ".fullschema" also includes dumps of the statistics tables "sqlite_stat1", "sqlite_stat3", and "sqlite_stat4", if they exist. The ".fullschema" command normally provides all of the information needed to exactly recreate a query plan for a specific query. When reporting suspected problems with the SQLite query planner to the SQLite development team, developers are requested to provide the complete ".fullschema" output as part of the trouble report. Note that the sqlite_stat3 and sqlite_stat4 tables contain samples of index entries and so might contain sensitive data, so do not send the ".fullschema" output of a proprietary database over a public channel.