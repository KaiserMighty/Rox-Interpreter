# Rox Programming Language
Dynamically-typed language with automatic memory management, comes with an interpreter and compiler. Tree-walk interpreter written in Java, directly traverses AST and evaluates exprs and stmts during runtime. Compiler written in C, translates source code into bytecode and executes it on a VM, custom garbage collector. Derived from Crafting Interpreters by Robert Nystrom.

## Java Build
### Generate Boilerplate Code First (if Expr.java and Stmt.java do not exist):
Compile: `javac interpreter/GenerateAst.java`  
Run: `java interpreter.GenerateAst interpreter`  

### Compile and Run Project
Compile: `javac interpreter/*.java`  
Run (Prompt): `java interpreter.Rox`  
Run (File): `java interpreter.Rox test.rox`

## C Build 
### Compile and Run Project
Compile: `make`  
Clear: `make clean`  
Run (Prompt): `./rox`  
Run (File): `./rox test.rox`

## Syntax Grammer
```
program          → declaration* EOF ;
```
### Declarations
```
declaration         → classDecl
                    | funDecl
                    | varDecl
                    | statement ;

classDecl           → "class" IDENTIFIER ( "<" IDENTIFIER )? "{" function* "}" ;
funDecl             → "fun" function ;
varDecl             → "var" IDENTIFIER ( "=" expression )? ";" ;
```
### Statements
```
statement           → exprStmt
                    | forStmt
                    | ifStmt
                    | printStmt
                    | returnStmt
                    | whileStmt
                    | block ;

exprStmt            → expression ";" ;
forStmt             → "for" "(" ( varDecl | exprStmt | ";" ) expression? ";" expression? ")" statement ;
ifStmt              → "if" "(" expression ")" statement ( "else" statement )? ;
printStmt           → "print" expression ";" ;
returnStmt          → "return" expression? ";" ;
whileStmt           → "while" "(" expression ")" statement ;
block               → "{" declaration* "}" ;
```
### Expressions
```
expression          → assignment ;

assignment          → ( call "." )? IDENTIFIER "=" assignment | logic_or ;

logic_or            → logic_and ( "or" logic_and )* ;
logic_and           → equality ( "and" equality )* ;
equality            → comparison ( ( "!=" | "==" ) comparison )* ;
comparison          → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term                → factor ( ( "-" | "+" ) factor )* ;
factor              → unary ( ( "/" | "*" ) unary )* ;
unary               → ( "!" | "-" ) unary | call ;
call                → primary ( "(" arguments? ")" | "." IDENTIFIER )* ;
primary             → "true" | "false" | "nil" | "this"
                    | NUMBER | STRING | IDENTIFIER | "(" expression ")"
                    | "super" "." IDENTIFIER ;
```
### Utility Rules
```
function            → IDENTIFIER "(" parameters? ")" block ;
parameters          → IDENTIFIER ( "," IDENTIFIER )* ;
arguments           → expression ( "," expression )* ;
```
### Lexical Grammer
```
NUMBER              → DIGIT+ ( "." DIGIT+ )? ;
STRING              → "\"" <any char except "\"">* "\"" ;
IDENTIFIER          → ALPHA ( ALPHA | DIGIT )* ;
ALPHA               → "a" ... "z" | "A" ... "Z" | "_" ;
DIGIT               → "0" ... "9" ;
```