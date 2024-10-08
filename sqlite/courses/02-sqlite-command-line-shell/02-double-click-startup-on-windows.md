
# 2. Double-click Startup On Windows

    - Windows users can double-click on the sqlite3.exe icon to cause the command-line shell to pop-up a terminal window running SQLite. However, because double-clicking starts the sqlite3.exe without command-line arguments, no database file will have been specified, so SQLite will use a temporary database that is deleted when the session exits. To use a persistent disk file as the database, enter the ".open" command immediately after the terminal window starts up:

        SQLite version 3.36.0 2021-06-18 18:36:39
        Enter ".help" for usage hints.
        Connected to a transient in-memory database.
        Use ".open FILENAME" to reopen on a persistent database.
        sqlite> .open ex1.db
        sqlite>

    - The example above causes the database file named "ex1.db" to be opened and used. The "ex1.db" file is created if it does not previously exist. You might want to use a full pathname to ensure that the file is in the directory that you think it is in. Use forward-slashes as the directory separator character. In other words use "c:/work/ex1.db", not "c:\work\ex1.db".

    - Alternatively, you can create a new database using the default temporary storage, then save that database into a disk file using the ".save" command:

        SQLite version 3.36.0 2021-06-18 18:36:39
        Enter ".help" for usage hints.
        Connected to a transient in-memory database.
        Use ".open FILENAME" to reopen on a persistent database.
        sqlite> ... many SQL commands omitted ...
        sqlite> .save ex1.db
        sqlite>

    - Be careful when using the ".save" command as it will overwrite any preexisting database files having the same name without prompting for confirmation. As with the ".open" command, you might want to use a full pathname with forward-slash directory separators to avoid ambiguity.

