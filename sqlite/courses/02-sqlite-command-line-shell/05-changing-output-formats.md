
# Changing Output Formats

    - The sqlite3 program is able to show the results of a query in 14 different output formats:

        ascii
        box
        csv
        column
        html
        insert
        json
        line
        list
        markdown
        quote
        table
        tabs
        tcl

    - You can use the ".mode" dot command to switch between these output formats. The default output mode is "list". In list mode, each row of a query result is written on one line of output and each column within that row is separated by a specific separator string. The default separator is a pipe symbol ("|"). List mode is especially useful when you are going to send the output of a query to another program (such as AWK) for additional processing.

        sqlite> .mode list
        sqlite> select * from tbl1;
        hello!|10
        goodbye|20
        sqlite>

    - Type ".mode" with no arguments to show the current mode:

        sqlite> .mode
        current output mode: list
        sqlite>

    - Use the ".separator" dot command to change the separator. For example, to change the separator to a comma and a space, you could do this:

        sqlite> .separator ", "
        sqlite> select * from tbl1;
        hello!, 10
        goodbye, 20
        sqlite>

    - The next ".mode" command might reset the ".separator" back to some default value (depending on its arguments). So you will likely need to repeat the ".separator" command whenever you change modes if you want to continue using a non-standard separator.

    - In "quote" mode, the output is formatted as SQL literals. Strings are enclosed in single-quotes and internal single-quotes are escaped by doubling. Blobs are displayed in hexadecimal blob literal notation (Ex: x'abcd'). Numbers are displayed as ASCII text and NULL values are shown as "NULL". All columns are separated from each other by a comma (or whatever alternative character is selected using ".separator").

        sqlite> .mode quote
        sqlite> select * from tbl1;
        'hello!',10
        'goodbye',20
        sqlite>

    - In "line" mode, each column in a row of the database is shown on a line by itself. Each line consists of the column name, an equal sign and the column data. Successive records are separated by a blank line. Here is an example of line mode output:

        sqlite> .mode line
        sqlite> select * from tbl1;
        one = hello!
        two = 10

        one = goodbye
        two = 20
        sqlite>

    - In column mode, each record is shown on a separate line with the data aligned in columns. For example:

        sqlite> .mode column
        sqlite> select * from tbl1;
        one       two
        --------  ---
        hello!    10
        goodbye   20
        sqlite>

    - In "column" mode (and also in "box", "table", and "markdown" modes) the width of columns adjusts automatically. But you can override this, providing a specified width for each column using the ".width" command. The arguments to ".width" are integers which are the number of characters to devote to each column. Negative numbers mean right-justify. Thus:

        sqlite> .width 12 -6
        sqlite> select * from tbl1;
        one              two
        ------------  ------
        hello!            10
        goodbye           20
        sqlite>

    - A width of 0 means the column width is chosen automatically. Unspecified column widths become zero. Hence, the command ".width" with no arguments resets all column widths to zero and hence causes all column widths to be determined automatically.

    The "column" mode is a tabular output format. Other tabular output formats are "box", "markdown", and "table":

        sqlite> .width
        sqlite> .mode markdown
        sqlite> select * from tbl1;
        |   one   | two |
        |---------|-----|
        | hello!  | 10  |
        | goodbye | 20  |
        sqlite> .mode table
        sqlite> select * from tbl1;
        +---------+-----+
        |   one   | two |
        +---------+-----+
        | hello!  | 10  |
        | goodbye | 20  |
        +---------+-----+
        sqlite> .mode box
        sqlite> select * from tbl1;
        ┌─────────┬─────┐
        │   one   │ two │
        ├─────────┼─────┤
        │ hello!  │ 10  │
        │ goodbye │ 20  │
        └─────────┴─────┘
        sqlite>

    - The columnar modes accept some addition options to control formatting. The "--wrap N" option (where N is an integer) causes columns to wrap text that is longer than N characters. Wrapping is disabled if N is zero.

        sqlite> insert into tbl1 values('The quick fox jumps over a lazy brown dog.',90);
        sqlite> .mode box --wrap 30
        sqlite> select * from tbl1 where two>50;
        ┌────────────────────────────────┬─────┐
        │              one               │ two │
        ├────────────────────────────────┼─────┤
        │ The quick fox jumps over a laz │ 90  │
        │ y brown dog.                   │     │
        └────────────────────────────────┴─────┘
        sqlite>

    - Wrapping happens after exactly N characters, which might be in the middle of a word. To wrap at a word boundary, add the "--wordwrap on" option (or just "-ww" for short):

        sqlite> .mode box --wrap 30 -ww
        sqlite> select * from tbl1 where two>50;
        ┌─────────────────────────────┬─────┐
        │             one             │ two │
        ├─────────────────────────────┼─────┤
        │ The quick fox jumps over a  │ 90  │
        │ lazy brown dog.             │     │
        └─────────────────────────────┴─────┘
        sqlite>

    - The "--quote" option causes the results in each column to be quoted like an SQL literal, as in the "quote" mode. See the on-line help for additional options.

    - The command ".mode box --wrap 60 --quote" is so useful for general-purpose database queries that it is given its own alias. Instead of typing out that whole 27-character command, you can just say ".mode qbox".

    - Another useful output mode is "insert". In insert mode, the output is formatted to look like SQL INSERT statements. Use insert mode to generate text that can later be used to input data into a different database.

    - When specifying insert mode, you have to give an extra argument which is the name of the table to be inserted into. For example:

        sqlite> .mode insert new_table
        sqlite> select * from tbl1 where two<50;
        INSERT INTO "new_table" VALUES('hello',10);
        INSERT INTO "new_table" VALUES('goodbye',20);
        sqlite>

    - Other output modes include "csv", "json", and "tcl". Try these yourself to see what they do.