grammar Lox;
program: statement* EOF;

statement: printStmt;

printStmt: 'print' exp = term ';';
term: STRING # strval | NUMBER # numval;
WS: [ \t\r\n]+ -> skip; //channel(HIDDEN);
NUMBER: DIGIT+ ( '.' DIGIT+)?;
STRING: '"' .*? '"';
DIGIT: '0' .. '9';