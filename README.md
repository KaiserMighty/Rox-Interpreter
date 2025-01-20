# Rox Programming Language
Placeholder

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