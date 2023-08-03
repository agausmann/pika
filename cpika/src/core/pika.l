%{
    #include "pika.tab.h"
    #include "lex.h"
%}
DIGIT [0-9]
NZDIGIT [1-9]
BINDIGIT [0-1]
OCTDIGIT [0-7]
HEXDIGIT [0-9A-Fa-f]

ALPHA [A-Za-z]
IDENT_START [_A-Za-z]
IDENT_CHAR [_A-Za-z0-9]

IDENT {IDENT_START}{IDENT_CHAR}*

WHITESPACE [ \t\n\r]
LINE_COMMENT "//"[^\n]*\n
BLOCK_COMMENT "/*"[^\*]*(\*[^/][^\*]*)*"*/"

INT_LITERAL (0|{NZDIGIT}{DIGIT}*)

SEP ({WHITESPACE}|{LINE_COMMENT}|{BLOCK_COMMENT})+

%%

"->"  { return R_ARROW; }
".."  { return DOT2; }
"::"  { return COLON2; }
"=="  { return EQ2; }
"&&"  { return AND2; }
"+"   { return PLUS; }
"-"   { return MINUS; }
"."   { return DOT; }
","   { return COMMA; }
":"   { return COLON; }
";"   { return SCLN; }
"="   { return EQ; }
"!"   { return BANG; }
"("   { return L_PAR; }
")"   { return R_PAR; }
"["   { return L_BRK; }
"]"   { return R_BRK; }
"{"   { return L_BRC; }
"}"   { return R_BRC; }

{INT_LITERAL} { return INT_LITERAL; }
{IDENT} { return ident_or_keyword(yytext); }

{SEP} /* ignored */

%%
