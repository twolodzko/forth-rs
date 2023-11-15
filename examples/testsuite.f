\ The test suite from Annex F: Test Suite (with adaptations)
\ See: https://forth-standard.org/standard/testsuite

\ Original licening:

\ This is the source for the ANS test harness, it is based on the
\ harness originally developed by John Hayes

\ (C) 1995 JOHNS HOPKINS UNIVERSITY / APPLIED PHYSICS LABORATORY
\ MAY BE DISTRIBUTED FREELY AS LONG AS THIS COPYRIGHT NOTICE REMAINS.
\ VERSION 1.1

\ Revision history and possibly newer versions can be found at
\ http://www.forth200x/tests/ttester.fs

\ The harness was simplified and adapted to work with this implementation.

2147483647 CONSTANT MAX-INT
-2147483648 CONSTANT MIN-INT
MAX-INT CONSTANT MAX-UINT
-1 CONSTANT <TRUE>
0 CONSTANT <FALSE>

VARIABLE ACTUAL-DEPTH \ stack record
CREATE ACTUAL-RESULTS 20 CELLS ALLOT
VARIABLE START-DEPTH

: U< < ;  \ since everything is signed
: ERROR  ( -- ) 1 0 / ;

: T{ \ ( -- ) record the pre-test depth.
   DEPTH START-DEPTH ! ;

: -> \ ( ... -- ) record depth and contents of stack.
   DEPTH DUP ACTUAL-DEPTH !	\ record depth
   START-DEPTH @ > IF	\ if there is something on the stack
     DEPTH START-DEPTH @ - 0 DO \ save them
       ACTUAL-RESULTS I CELLS + !
     LOOP
   THEN ;

: }T \ ( ... -- ) comapre stack (expected) contents with saved
   \ (actual) contents.
   DEPTH ACTUAL-DEPTH @ = IF	         \ if depths match
     DEPTH START-DEPTH @ > IF	         \ if something on the stack
       DEPTH START-DEPTH @ - 0 DO	    \ for each stack item
         ACTUAL-RESULTS I CELLS + @	   \ compare actual with expected
         <> IF ERROR LEAVE THEN
       LOOP
     THEN
   ELSE                                    \ depth mismatch
     ERROR
   THEN ;

\ F.6.1.0080 (
\ There is no space either side of the ).
T{ ( A comment)1234 -> 1234 }T  \ output changed to 1234, Gforth does the same
T{ : pc1 ( A comment)1234 ; pc1 -> 1234 }T

\ F.6.1.0090 *
T{  0  0 * ->  0 }T          \ TEST IDENTITIE\S
T{  0  1 * ->  0 }T
T{  1  0 * ->  0 }T
T{  1  2 * ->  2 }T
T{  2  1 * ->  2 }T
T{  3  3 * ->  9 }T
T{ -3  3 * -> -9 }T
T{  3 -3 * -> -9 }T
T{ -3 -3 * ->  9 }T

\ F.6.1.0100 */
\ T{       0 2       1 */ ->       0 }T
\ T{       1 2       1 */ ->       1 }T
\ T{       2 2       1 */ ->       2 }T
\ T{      -1 2       1 */ ->      -1 }T
\ T{      -2 2       1 */ ->      -2 }T
\ T{       0 2      -1 */ ->       0 }T
\ T{       1 2      -1 */ ->       1 }T
\ T{       2 2      -1 */ ->       2 }T
\ T{      -1 2      -1 */ ->      -1 }T
\ T{      -2 2      -1 */ ->      -2 }T
\ T{       2 2       2 */ ->       2 }T
\ T{      -1 2      -1 */ ->      -1 }T
\ T{      -2 2      -2 */ ->      -2 }T
\ T{       7 2       3 */ ->       7 }T
\ T{       7 2      -3 */ ->       7 }T
\ T{      -7 2       3 */ ->      -7 }T
\ T{      -7 2      -3 */ ->      -7 }T
\ T{ MAX-INT 2 MAX-INT */ -> MAX-INT }T
\ T{ MIN-INT 2 MIN-INT */ -> MIN-INT }T

\ \ F.6.1.0110 */MOD
\ T{       0 2       1 */MOD ->       0 2 }T
\ T{       1 2       1 */MOD ->       1 2 }T
\ T{       2 2       1 */MOD ->       2 2 }T
\ T{      -1 2       1 */MOD ->      -1 2 }T
\ T{      -2 2       1 */MOD ->      -2 2 }T
\ T{       0 2      -1 */MOD ->       0 2 }T
\ T{       1 2      -1 */MOD ->       1 2 }T
\ T{       2 2      -1 */MOD ->       2 2 }T
\ T{      -1 2      -1 */MOD ->      -1 2 }T
\ T{      -2 2      -1 */MOD ->      -2 2 }T
\ T{       2 2       2 */MOD ->       2 2 }T
\ T{      -1 2      -1 */MOD ->      -1 2 }T
\ T{      -2 2      -2 */MOD ->      -2 2 }T
\ T{       7 2       3 */MOD ->       7 2 }T
\ T{       7 2      -3 */MOD ->       7 2 }T
\ T{      -7 2       3 */MOD ->      -7 2 }T
\ T{      -7 2      -3 */MOD ->      -7 2 }T
\ T{ MAX-INT 2 MAX-INT */MOD -> MAX-INT 2 }T
\ T{ MIN-INT 2 MIN-INT */MOD -> MIN-INT 2 }T

\ F.6.1.0120 +
T{        0  5 + ->          5 }T
T{        5  0 + ->          5 }T
T{        0 -5 + ->         -5 }T
T{       -5  0 + ->         -5 }T
T{        1  2 + ->          3 }T
T{        1 -2 + ->         -1 }T
T{       -1  2 + ->          1 }T
T{       -1 -2 + ->         -3 }T
T{       -1  1 + ->          0 }T

\ F.6.1.0150 ,
HERE 1 ,
HERE 2 ,
CONSTANT 2ND
CONSTANT 1ST
T{       1ST 2ND U< -> <TRUE> }T \ HERE MUST GROW WITH ALLOT
T{       1ST CELL+  -> 2ND }T \ ... BY ONE CELL
T{   1ST 1 CELLS +  -> 2ND }T
T{     1ST @ 2ND @  -> 1 2 }T
T{         5 1ST !  ->     }T
T{     1ST @ 2ND @  -> 5 2 }T
T{         6 2ND !  ->     }T
T{     1ST @ 2ND @  -> 5 6 }T
T{           1ST 2@ -> 6 5 }T
T{       2 1 1ST 2! ->     }T
T{           1ST 2@ -> 2 1 }T
T{ 1  1ST !  1ST @  -> 1   }T    \ CAN STORE CELL-WIDE VALUE

\ F.6.1.0160 -
T{          0  5 - ->       -5 }T
T{          5  0 - ->        5 }T
T{          0 -5 - ->        5 }T
T{         -5  0 - ->       -5 }T
T{          1  2 - ->       -1 }T
T{          1 -2 - ->        3 }T
T{         -1  2 - ->       -3 }T
T{         -1 -2 - ->        1 }T
T{          0  1 - ->       -1 }T

\ F.6.1.0190 ."
T{ : pb1 CR ." You should see 2345: "." 2345"; pb1 -> }T

\ \ F.6.1.0230 /
\ T{       0       1 / ->       0 }T
\ T{       1       1 / ->       1 }T
\ T{       2       1 / ->       2 }T
\ T{      -1       1 / ->      -1 }T
\ T{      -2       1 / ->      -2 }T
\ T{       0      -1 / ->       0 }T
\ T{       1      -1 / ->       1 }T
\ T{       2      -1 / ->       2 }T
\ T{      -1      -1 / ->      -1 }T
\ T{      -2      -1 / ->      -2 }T
\ T{       2       2 / ->       2 }T
\ T{      -1      -1 / ->      -1 }T
\ T{      -2      -2 / ->      -2 }T
\ T{       7       3 / ->       7 }T
\ T{       7      -3 / ->       7 }T
\ T{      -7       3 / ->      -7 }T
\ T{      -7      -3 / ->      -7 }T
\ T{ MAX-INT       1 / -> MAX-INT }T
\ T{ MIN-INT       1 / -> MIN-INT }T
\ T{ MAX-INT MAX-INT / -> MAX-INT }T
\ T{ MIN-INT MIN-INT / -> MIN-INT }T

\ \ F.6.1.0240 /MOD
\ T{       0       1 /MOD ->       0 }T
\ T{       1       1 /MOD ->       1 }T
\ T{       2       1 /MOD ->       2 }T
\ T{      -1       1 /MOD ->      -1 }T
\ T{      -2       1 /MOD ->      -2 }T
\ T{       0      -1 /MOD ->       0 }T
\ T{       1      -1 /MOD ->       1 }T
\ T{       2      -1 /MOD ->       2 }T
\ T{      -1      -1 /MOD ->      -1 }T
\ T{      -2      -1 /MOD ->      -2 }T
\ T{       2       2 /MOD ->       2 }T
\ T{      -1      -1 /MOD ->      -1 }T
\ T{      -2      -2 /MOD ->      -2 }T
\ T{       7       3 /MOD ->       7 }T
\ T{       7      -3 /MOD ->       7 }T
\ T{      -7       3 /MOD ->      -7 }T
\ T{      -7      -3 /MOD ->      -7 }T
\ T{ MAX-INT       1 /MOD -> MAX-INT }T
\ T{ MIN-INT       1 /MOD -> MIN-INT }T
\ T{ MAX-INT MAX-INT /MOD -> MAX-INT }T
\ T{ MIN-INT MIN-INT /MOD -> MIN-INT }T

\ F.6.1.0270 0=
T{        0 0= -> <TRUE>  }T
T{        1 0= -> <FALSE> }T
T{        2 0= -> <FALSE> }T
T{       -1 0= -> <FALSE> }T
T{ MAX-UINT 0= -> <FALSE> }T
T{ MIN-INT  0= -> <FALSE> }T
T{ MAX-INT  0= -> <FALSE> }T

\ F.6.1.0290 1+
T{        0 1+ ->          1 }T
T{       -1 1+ ->          0 }T
T{        1 1+ ->          2 }T

\ F.6.1.0300 1-
T{          2 1- ->        1 }T
T{          1 1- ->        0 }T
T{          0 1- ->       -1 }T

\ F.6.1.0320 2*
T{    0 2*       ->    0 }T
T{    1 2*       ->    2 }T
T{ 4000 2*       -> 8000 }T
\ T{    1 2* 1 XOR ->    1 }T
\ T{  MSB 2*       ->    0 }T

\ F.6.1.0330 2/
T{           0 2/ ->    0 }T
T{           1 2/ ->    0 }T
T{        4000 2/ -> 2000 }T
\ T{           1 2/ ->    1 }T \ MSB PROPOGATED
\ T{     1 1 XOR 2/ ->    1 }T
\ T{ MSB 2/ MSB AND ->  MSB }T

\ F.6.1.0370 2DROP
T{ 1 2 2DROP -> }T

\ F.6.1.0380 2DUP
T{ 1 2 2DUP -> 1 2 1 2 }T

\ F.6.1.0400 2OVER
T{ 1 2 3 4 2OVER -> 1 2 3 4 1 2 }T

\ F.6.1.0430 2SWAP
T{ 1 2 3 4 2SWAP -> 3 4 1 2 }T

\ F.6.1.0480 <
T{       0       1 < -> <TRUE>  }T
T{       1       2 < -> <TRUE>  }T
T{      -1       0 < -> <TRUE>  }T
T{      -1       1 < -> <TRUE>  }T
T{ MIN-INT       0 < -> <TRUE>  }T
T{ MIN-INT MAX-INT < -> <TRUE>  }T
T{       0 MAX-INT < -> <TRUE>  }T
T{       0       0 < -> <FALSE> }T
T{       1       1 < -> <FALSE> }T
T{       1       0 < -> <FALSE> }T
T{       2       1 < -> <FALSE> }T
T{       0      -1 < -> <FALSE> }T
T{       1      -1 < -> <FALSE> }T
T{       0 MIN-INT < -> <FALSE> }T
T{ MAX-INT MIN-INT < -> <FALSE> }T
T{ MAX-INT       0 < -> <FALSE> }T

\ F.6.1.0530 =
T{  0  0 = -> <TRUE>  }T
T{  1  1 = -> <TRUE>  }T
T{ -1 -1 = -> <TRUE>  }T
T{  1  0 = -> <FALSE> }T
T{ -1  0 = -> <FALSE> }T
T{  0  1 = -> <FALSE> }T
T{  0 -1 = -> <FALSE> }T

\ F.6.1.0540 >
T{       0       1 > -> <FALSE> }T
T{       1       2 > -> <FALSE> }T
T{      -1       0 > -> <FALSE> }T
T{      -1       1 > -> <FALSE> }T
T{ MIN-INT       0 > -> <FALSE> }T
T{ MIN-INT MAX-INT > -> <FALSE> }T
T{       0 MAX-INT > -> <FALSE> }T
T{       0       0 > -> <FALSE> }T
T{       1       1 > -> <FALSE> }T
T{       1       0 > -> <TRUE>  }T
T{       2       1 > -> <TRUE>  }T
T{       0      -1 > -> <TRUE>  }T
T{       1      -1 > -> <TRUE>  }T
T{       0 MIN-INT > -> <TRUE>  }T
T{ MAX-INT MIN-INT > -> <TRUE>  }T
T{ MAX-INT       0 > -> <TRUE>  }T

\ F.6.1.0580 >R
T{ : GR1 >R R> ; -> }T
T{ : GR2 >R R@ R> DROP ; -> }T
T{ 123 GR1 -> 123 }T
T{ 123 GR2 -> 123 }T
T{  1 GR1 ->  1 }T      ( Return stack holds cells )

\ F.6.1.0690 ABS
T{       0 ABS ->          0 }T
T{       1 ABS ->          1 }T
T{      -1 ABS ->          1 }T

\ F.6.1.0720 AND
T{ 0 0 AND -> 0 }T
T{ 0 1 AND -> 0 }T
T{ 1 0 AND -> 0 }T
T{ 1 1 AND -> 1 }T
T{ 0 INVERT 1 AND -> 1 }T
T{ 1 INVERT 1 AND -> 0 }T
T{ 0 0 AND -> 0 }T
T{ 0 1 AND -> 0 }T
T{ 1 0 AND -> 0 }T
T{ 1 1 AND -> 1 }T

\ \ F.6.1.1720 INVERT
\ T{ 0 INVERT -> 1 }T
\ T{ 1 INVERT -> 0 }T

\ F.6.1.0860 C,
HERE 1 C,
HERE 2 C,
CONSTANT 2NDC
CONSTANT 1STC
T{    1STC 2NDC U< -> <TRUE> }T	\ HERE MUST GROW WITH ALLOT
T{      1STC CHAR+ ->  2NDC  }T	\ ... BY ONE CHAR
T{  1STC 1 CHARS + ->  2NDC  }T
T{ 1STC C@ 2NDC C@ ->   1 2  }T
T{       3 1STC C! ->        }T
T{ 1STC C@ 2NDC C@ ->   3 2  }T
T{       4 2NDC C! ->        }T
T{ 1STC C@ 2NDC C@ ->   3 4  }T

\ F.6.1.0890 CELLS
\ : BITS ( X -- U )
\    0 SWAP BEGIN DUP WHILE
\      DUP MSB AND IF >R 1+ R> THEN 2*
\    REPEAT DROP ;
( CELLS >= 1 AU, INTEGRAL MULTIPLE OF CHAR SIZE, >= 16 BITS )
T{ 1 CELLS 1 <         -> <FALSE> }T
T{ 1 CELLS 1 CHARS MOD ->    0    }T
\ T{ 1 BITS 10 <        -> <FALSE> }T

\ F.6.1.0950 CONSTANT
T{ 123 CONSTANT X123 -> }T
T{ X123 -> 123 }T
\ T{ : EQU CONSTANT ; -> }T
\ T{ X123 EQU Y123 -> }T
\ T{ Y123 -> 123 }T

\ F.6.1.1200 DEPTH
T{ 0 1 DEPTH -> 0 1 2 }T
T{   0 DEPTH -> 0 1   }T
T{     DEPTH -> 0     }T

\ F.6.1.1260 DROP
T{ 1 2 DROP -> 1 }T
T{ 0   DROP ->   }T

\ F.6.1.1290 DUP
T{ 1 DUP -> 1 1 }T

\ F.6.1.1700 IF
T{ : GI1 IF 123 THEN ; -> }T
T{ : GI2 IF 123 ELSE 234 THEN ; -> }T
T{  0 GI1 ->     }T
T{  1 GI1 -> 123 }T
T{ -1 GI1 -> 123 }T
T{  0 GI2 -> 234 }T
T{  1 GI2 -> 123 }T
T{ -1 GI1 -> 123 }T

\ F.6.1.1760 LEAVE
T{ : GD5 123 SWAP 0 DO
     I 4 > IF DROP 234 LEAVE THEN
   LOOP ; -> }T
T{ 1 GD5 -> 123 }T
T{ 5 GD5 -> 123 }T
T{ 6 GD5 -> 234 }T

\ F.6.1.1800 LOOP
T{ : GD1 DO I LOOP ; -> }T
T{          4        1 GD1 ->  1 2 3   }T
T{          2       -1 GD1 -> -1 0 1   }T

\ F.6.1.1870 MAX
T{       0       1 MAX ->       1 }T
T{       1       2 MAX ->       2 }T
T{      -1       0 MAX ->       0 }T
T{      -1       1 MAX ->       1 }T
T{ MIN-INT       0 MAX ->       0 }T
T{ MIN-INT MAX-INT MAX -> MAX-INT }T
T{       0 MAX-INT MAX -> MAX-INT }T
T{       0       0 MAX ->       0 }T
T{       1       1 MAX ->       1 }T
T{       1       0 MAX ->       1 }T
T{       2       1 MAX ->       2 }T
T{       0      -1 MAX ->       0 }T
T{       1      -1 MAX ->       1 }T
T{       0 MIN-INT MAX ->       0 }T
T{ MAX-INT MIN-INT MAX -> MAX-INT }T
T{ MAX-INT       0 MAX -> MAX-INT }T

\ F.6.1.1880 MIN
T{       0       1 MIN ->       0 }T
T{       1       2 MIN ->       1 }T
T{      -1       0 MIN ->      -1 }T
T{      -1       1 MIN ->      -1 }T
T{ MIN-INT       0 MIN -> MIN-INT }T
T{ MIN-INT MAX-INT MIN -> MIN-INT }T
T{       0 MAX-INT MIN ->       0 }T
T{       0       0 MIN ->       0 }T
T{       1       1 MIN ->       1 }T
T{       1       0 MIN ->       0 }T
T{       2       1 MIN ->       1 }T
T{       0      -1 MIN ->      -1 }T
T{       1      -1 MIN ->      -1 }T
T{       0 MIN-INT MIN -> MIN-INT }T
T{ MAX-INT MIN-INT MIN -> MIN-INT }T
T{ MAX-INT       0 MIN ->       0 }T

\ \ F.6.1.1890 MOD
\ T{       0       1 MOD ->       0 }T
\ T{       1       1 MOD ->       1 }T
\ T{       2       1 MOD ->       2 }T
\ T{      -1       1 MOD ->      -1 }T
\ T{      -2       1 MOD ->      -2 }T
\ T{       0      -1 MOD ->       0 }T
\ T{       1      -1 MOD ->       1 }T
\ T{       2      -1 MOD ->       2 }T
\ T{      -1      -1 MOD ->      -1 }T
\ T{      -2      -1 MOD ->      -2 }T
\ T{       2       2 MOD ->       2 }T
\ T{      -1      -1 MOD ->      -1 }T
\ T{      -2      -2 MOD ->      -2 }T
\ T{       7       3 MOD ->       7 }T
\ T{       7      -3 MOD ->       7 }T
\ T{      -7       3 MOD ->      -7 }T
\ T{      -7      -3 MOD ->      -7 }T
\ T{ MAX-INT       1 MOD -> MAX-INT }T
\ T{ MIN-INT       1 MOD -> MIN-INT }T
\ T{ MAX-INT MAX-INT MOD -> MAX-INT }T
\ T{ MIN-INT MIN-INT MOD -> MIN-INT }T

\ F.6.1.1910 NEGATE
T{  0 NEGATE ->  0 }T
T{  1 NEGATE -> -1 }T
T{ -1 NEGATE ->  1 }T
T{  2 NEGATE -> -2 }T
T{ -2 NEGATE ->  2 }T

\ F.6.1.1980 OR
T{ 0 0 OR -> 0 }T
T{ 0 1 OR -> 1 }T
T{ 1 0 OR -> 1 }T
T{ 1 1 OR -> 1 }T

\ F.6.1.1990 OVER
T{ 1 2 OVER -> 1 2 1 }T

\ \ F.6.1.2120 RECURSE
\ T{ : GI6 ( N -- 0,1,..N )
\      DUP IF DUP >R 1- RECURSE R> THEN ; -> }T
\ T{ 0 GI6 -> 0 }T
\ T{ 1 GI6 -> 0 1 }T
\ T{ 2 GI6 -> 0 1 2 }T
\ T{ 3 GI6 -> 0 1 2 3 }T
\ T{ 4 GI6 -> 0 1 2 3 4 }T

\ F.6.1.2260 SWAP
T{ 1 2 SWAP -> 2 1 }T

\ F.6.1.2390 UNTIL
T{ : GI4 BEGIN DUP 1+ DUP 5 > UNTIL ; -> }T
T{ 3 GI4 -> 3 4 5 6 }T
T{ 5 GI4 -> 5 6 }T
T{ 6 GI4 -> 6 7 }T

\ F.6.1.2410 VARIABLE
T{ VARIABLE V1 ->     }T
T{    123 V1 ! ->     }T
T{        V1 @ -> 123 }T

\ F.6.1.2430 WHILE
T{ : GI3 BEGIN DUP 5 < WHILE DUP 1+ REPEAT ; -> }T
T{ 0 GI3 -> 0 1 2 3 4 5 }T
T{ 4 GI3 -> 4 5 }T
T{ 5 GI3 -> 5 }T
T{ 6 GI3 -> 6 }T
\ T{ : GI5 BEGIN DUP 2 > WHILE
\       DUP 5 < WHILE DUP 1+ REPEAT
\       123 ELSE 345 THEN ; -> }T
\ T{ 1 GI5 -> 1 345 }T
\ T{ 2 GI5 -> 2 345 }T
\ T{ 3 GI5 -> 3 4 5 123 }T
\ T{ 4 GI5 -> 4 5 123 }T
\ T{ 5 GI5 -> 5 123 }T

\ F.6.1.2490 XOR
T{ 0 0 XOR -> 0 }T
T{ 0 1 XOR -> 1 }T
T{ 1 0 XOR -> 1 }T
T{ 1 1 XOR -> 0 }T

\ F.6.2.1485 FALSE
T{ FALSE -> 0 }T
T{ FALSE -> <FALSE> }T

\ F.6.2.2298 TRUE
T{ TRUE -> <TRUE> }T
T{ TRUE -> 0 INVERT }T

\ F.6.2.2405 VALUE
\ v1 and v2 in the original test suite were renamed to u1 and u2
\ due to name collisions
T{  111 VALUE u1 -> }T
T{ -999 VALUE u2 -> }T
T{ u1 ->  111 }T
T{ u2 -> -999 }T
T{ 222 TO u1 -> }T
T{ u1 -> 222 }T
T{ : vd1 u1 ; -> }T
T{ vd1 -> 222 }T
T{ : vd2 TO u2 ; -> }T
T{ u2 -> -999 }T
T{ -333 vd2 -> }T
T{ u2 -> -333 }T
T{ u1 ->  222 }T

\ F.6.1.0710 ALLOT
HERE 1 ALLOT
HERE
CONSTANT 2NDA
CONSTANT 1STA
T{ 1STA 2NDA U< -> <TRUE> }T    \ HERE MUST GROW WITH ALLOT
T{      1STA 1+ ->   2NDA }T    \ ... BY ONE ADDRESS UNIT
( MISSING TEST: NEGATIVE ALLOT )

BYE
