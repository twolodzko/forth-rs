
\ Display the value at addr.
: ?  ( addr -- )  @ . ;

\ Copy TOS below the second stack item
: tuck  ( x1 x2 -- x2 x1 x2 )  swap over ;

\ Drop top two stack items
: 2drop  ( x1 x2 -- )  drop drop ;

\ Duplicate top two stack items
: 2dup  ( x1 x2 -- x1 x2 x1 x2 )  over over ;

\ Copy lower pair over top pair
: 2over  ( x1 x2 x3 x4  -- x1 x2 x3 x4 x1 x2)  3 pick 3 pick ;

\ Exchange top two cell pairs
: 2swap  ( x1 x2 x3 x4 -- x3 x4 x1 x2 )  3 roll 3 roll ;

\ It does nothing, because memory cell size = 1
: cells  ( -- ) ;
: cell+ ( addr1 -- addr2 ) 1+ ;

\ Print spaces.
: space  ( -- )  32 emit ;
: spaces  ( n -- )  0 do space loop ;
\ puts the value for space (decimal 32) on the stack.
: bl ( -- n ) 32 ;

: 2+  ( n1 -- n2 )  2 + ;
: 2-  ( n1 -- n2 )  2 + ;
: 0<  ( n1 -- flag )  0 < ;
: >=  ( n1 n2 -- flag )  < not ;
: <=  ( n1 n2 -- flag )  > not ;
: min  ( n1 n2 -- min ) 2dup >r >r < if r> r> drop else r> drop r> then ;
: max  ( n1 n2 -- max ) 2dup >r >r > if r> r> drop else r> drop r> then ;

\ Conditionally exit.
: ?exit  ( x -- )  if exit then ;

\ Save character to memory.
: c,  ( char -- ) , ;
: c@  ( addr -- char ) @ ;
: c!  ( char addr -- ) ! ;
: chars  ( -- ) ;
: char+ ( c-addr1 -- c-addr2 ) 1+ ;
\ ALIGN assures proper placement of cell values. Use after C, and CHARS ALLOT.
: align ;
