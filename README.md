# terminal-calculator (v.0.5.0)
A terminal-based calculator application, written in Rust.

## installation
To use this, you must at the moment build from source. 

Download the files (linux), go into the `/terminal-calculator` directory and execute the following commands:
```
cargo build --release
sudo cp ./target/release/calc /usr/bin
```

A cargo installable version will be available once this program is out of beta (v. 1.0.0).

## usage
There are two ways to use this program, through CLI args or as a REPL:

### CLI args
```
$ calc 2 + 2
Result: 4
$ _
```

**debug mode**
You can enable debug mode through the CLI with `--debug`, it may be both before or after the calculation, but not anywhere in the middle.
```
$ calc --debug 2 + 2
Debug mode enabled.

Input: 2 + 2
Tokenising...
Tokenisation complete.
Tokens:
Type: Number, Lexeme: 2
Type: Binary Operator, Addition, Lexeme: +
Type: Number, Lexeme: 2
Token printing complete.

Generating AST...
AST Generated.

Result: 4
$ _
```
Or:
```
$ calc 2 + 2 --debug
Debug mode enabled.

Input: 2 + 2
Tokenising...
Tokenisation complete.
Tokens:
Type: Number, Lexeme: 2
Type: Binary Operator, Addition, Lexeme: +
Type: Number, Lexeme: 2
Token printing complete.

Generating AST...
AST Generated.

Result: 4
$ _
```


### REPL mode
To enter the calculator as a REPL (Read-Execute-Print Loop) just use `calc` on its own.
```
$ calc
Welcome to my terminal calculator!
> _
```
You may now perform calculations here.
```
$ calc
Welcome to my terminal calculator!
> 2 + 2
Result: 4
> _
```
To exit, simply type "exit".
```
$ calc
Welcome to my terminal calculator!
> 2 + 2
Result: 4
> exit
$ _
```

**debug mode**
To toggle debug mode in the REPL, simply type "debug" or "dbg".
```
$ calc
Welcome to my terminal calculator!
> debug
Debug mode enabled.
> _
```
You will now see the process when you execute commands.
```
$ calc
Welcome to my terminal calculator!
> debug
Debug mode enabled.
> 2 + 2

Input: 2 + 2
Tokenising...
Tokenisation complete.
Tokens:
Type: Number, Lexeme: 2
Type: Binary Operator, Addition, Lexeme: +
Type: Number, Lexeme: 2
Token printing complete.

Generating AST...
AST Generated.

Result: 4
> _
```
"debug" and "dbg" are toggles, so to disable it again, simply type the command again.
```
$ calc
Welcome to my terminal calculator!
> debug
Debug mode enabled.
> 2 + 2

Input: 2 + 2
Tokenising...
Tokenisation complete.
Tokens:
Type: Number, Lexeme: 2
Type: Binary Operator, Addition, Lexeme: +
Type: Number, Lexeme: 2
Token printing complete.

Generating AST...
AST Generated.

Result: 4
> dbg
Debug mode disabled.
> _
```
It will now work as normal.
```
$ calc
Welcome to my terminal calculator!
> debug
Debug mode enabled.
> 2 + 2

Input: 2 + 2
Tokenising...
Tokenisation complete.
Tokens:
Type: Number, Lexeme: 2
Type: Binary Operator, Addition, Lexeme: +
Type: Number, Lexeme: 2
Token printing complete.

Generating AST...
AST Generated.

Result: 4
> dbg
Debug mode disabled.
> 2 + 2
Result: 4
> exit
$ _
```

