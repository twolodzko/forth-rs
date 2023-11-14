: fizz?  3 mod 0 = dup if ." Fizz" then ;
: buzz?  5 mod 0 = dup if ." Buzz" then ;
: fizz-buzz?  dup fizz? swap buzz? or not ;
: fizz-buzz  1 do i fizz-buzz? if i . then cr loop ;
25 fizz-buzz
