CREATE CHARACTER  CHAR A C, CHAR B C, ALIGN
CHARACTER C@ EMIT
CHARACTER CHAR+ C@ EMIT
CHAR Z CHARACTER C!
CHAR ! CHARACTER CHAR+ C!
CHARACTER C@ EMIT
CHARACTER CHAR+ C@ EMIT
: C!+  ( c-addr char -- c-addr+ )  OVER C! CHAR+ ;
\ C@+ and EMITS we'll meet again as COUNT and TYPE
: C@+  ( c-addr -- c-addr+ char )  DUP CHAR+ SWAP C@ ;
: EMITS  ( c-addr n )  0 MAX  0 DO  C@+ EMIT  LOOP  DROP ;
CREATE 5CHARS 5 CHARS ALLOT ALIGN
5CHARS
CHAR F C!+  CHAR o C!+  CHAR r C!+  CHAR t C!+  CHAR h C!+
DROP  \ drop the c-addr remaining after the last C!+

5CHARS 5 EMITS
