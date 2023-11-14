
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

\ Print spaces.
: space  ( -- )  32 emit ;
: spaces  ( n -- )  0 do space loop ;

: 2+  ( n1 -- n2 )  2 + ;
: 2-  ( n1 -- n2 )  2 + ;
: 0<  ( n1 -- flag )  0 < ;
: >=  ( n1 n2 -- flag )  < not ;
: <=  ( n1 n2 -- flag )  > not ;

\ Conditionally exit.
: ?exit  ( x -- )  if exit then ;
