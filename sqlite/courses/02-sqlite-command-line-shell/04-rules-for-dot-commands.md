
# Rules for "dot-commands", SQL and More


## 4.1. Line Structure

    - The CLI's input is parsed into a sequence consisting of:

        * SQL statements;
        * dot-commands; or
        * CLI comments

    - SQL statements are free-form, and can be spread across multiple lines, with whitespace or SQL comments embedded anywhere. They are terminated by either a ';' character at the end of an input line, or a '/' character or the word "go" on a line by itself. When not at the end of an input line, the ';' character acts to separate SQL statements. Trailing whitespace is ignored for purposes of termination.

    - A dot-command has a more restrictive structure:

        * It must begin with its "." at the left margin with no preceding whitespace.
        * It must be entirely contained on a single input line.
        * It cannot occur in the middle of an ordinary SQL statement. In other words, it cannot occur at a continuation prompt.
        * There is no comment syntax for dot-commands.

    - The CLI also accepts whole-line comments that begin with a '#' character and extend to the end of the line. There can be no with whitespace prior to the '#'.

## 4.2. Dot-command arguments

    - The arguments passed to dot-commands are parsed from the command tail, per these rules:

        1. The trailing newline and any other trailing whitespace is discarded;

        2. Whitespace immediately following the dot-command name, or any argument input end bound is discarded;
        
        3. An argument input begins with any non-whitespace character;
        
        4. An argument input ends with a character which depends upon its leading character thusly:

            * for a leading single-quote ('), a single-quote acts as the end delimiter;
            
            * for a leading double-quote ("), an unescaped double-quote acts as the end delimiter;
            
            * for any other leading character, the end delimiter is any whitespace; and
            
            * the command tail end acts as the end delimiter for any argument;

        5. Within a double-quoted argument input, a backslash-escaped double-quote is part of the argument rather than its terminating quote;

        6.Within a double-quoted argument, traditional C-string literal, backslash escape sequence translation is done; and
        
        7. Argument input delimiters (the bounding quotes or whitespace) are discarded to yield the passed argument.


## 4.3. Dot-command execution
    
    - The dot-commands are interpreted by the sqlite3.exe command-line program, not by SQLite itself. So none of the dot-commands will work as an argument to SQLite interfaces such as sqlite3_prepare() or sqlite3_exec().

