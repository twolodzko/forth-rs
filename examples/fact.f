: fac1 ( n -- n! )
    dup 0 > if
        dup 1- fac1 *
    else
        drop 1
    then ;

10 0 do i fac1 . cr loop

: fac2 ( n -- n! )
    dup 0 > if
        dup 1- recurse *
    else
        drop 1
    then ;

10 0 do i fac2 . cr loop
\ 10000 fac2
