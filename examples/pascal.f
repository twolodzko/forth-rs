\ Example source: http://www.murphywong.net/hello/simple.htm#L20

\ Pascal's Triangle
: POSITION  ( row -- )  CR  33 SWAP 2 *  - SPACES ;
: PAS ( 0 ... 0 -- 0 ... 0 )
   0 >R
   BEGIN  OVER + >R  DUP 0= UNTIL
   BEGIN  R> DUP WHILE  DUP 4 .R  REPEAT ;
: PASS  ( -- )
   0 1 0
   13 0 DO  DUP POSITION  >R  PAS  R>  1+  LOOP  DROP ;  \ using DO instead od ?DO here
: PAX  ( 0 ... 0 -- )  DROP BEGIN 0= UNTIL ;
: PASCAL  ( -- )  PASS PAX ;

PASCAL
