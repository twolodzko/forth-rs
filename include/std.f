
\ Display the value at addr.
: ?  ( addr -- )  @ . ;

\ Copy TOS below the second stack item
: tuck  ( x1 x2 -- x2 x1 x2 )  swap over ;
: nip  ( x1 x2 -- x2 ) swap drop ;

\ Drop top two stack items
: 2drop  ( x1 x2 -- )  drop drop ;

\ Duplicate top two stack items
: 2dup  ( x1 x2 -- x1 x2 x1 x2 )  over over ;

\ Copy lower pair over top pair
: 2over  ( x1 x2 x3 x4 -- x1 x2 x3 x4 x1 x2)  3 pick 3 pick ;

\ Exchange top two cell pairs
: 2swap  ( x1 x2 x3 x4 -- x3 x4 x1 x2 )  3 roll 3 roll ;

\ It does nothing, because memory cell size = 1
: cells  ( -- ) ;
: cell+ ( addr1 -- addr2 ) 1+ ;

\ Print spaces.
: space  ( -- )  32 emit ;
: spaces  ( n -- )  0 do space loop ;
\ Space (decimal 32)
: bl ( -- n ) 32 ;

: 2+  ( n1 -- n2 )  2 + ;
: 2-  ( n1 -- n2 )  2 + ;
: 0<  ( n1 -- flag )  0 < ;
: 0>  ( n1 -- flag )  0 > ;
: >=  ( n1 n2 -- flag )  < not ;
: <=  ( n1 n2 -- flag )  > not ;
: min  ( n1 n2 -- min ) 2dup < if drop else nip then ;
: max  ( n1 n2 -- max ) 2dup > if drop else nip then ;

\ Conditionally exit.
: ?exit  ( x -- )  if exit then ;
: ?dup  ( n -- n n ) dup if dup then ;

\ Save character to memory.
: c,  ( char -- ) , ;
: c@  ( addr -- char ) @ ;
: c!  ( char addr -- ) ! ;
: chars  ( -- ) ;
: char+ ( c-addr1 -- c-addr2 ) 1+ ;
\ ALIGN assures proper placement of cell values. Use after C, and CHARS ALLOT.
: align ;

: 2!  ( x1 x2 addr -- ) SWAP OVER ! CELL+ ! ;
: 2@  ( addr -- x1 x2 ) DUP CELL+ @ SWAP @ ;

: not  ( n1 -- n2 ) 0= ;

\ Add n to the value at addr
: +!  ( n addr -- ) swap over @ + swap ! ;

: u< < ;  \ since everything is signed
