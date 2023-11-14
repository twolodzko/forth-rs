\ Example source: http://www.murphywong.net/hello/simple.htm#L26

\ A year is a leap year if:
\ it's divisible by 4 but not by 100
\ or it's divisible by 400
: LEAP  ( year -- flag )  \ nonzero flag means a leap year
   DUP 4 MOD 0=  OVER 100 MOD AND
   SWAP 400 MOD 0= OR ;

: LEAP?  ( year -- )
   DUP .
   LEAP IF ." is" ELSE  ." isn't" THEN  ."  a leap year." ;

2000 LEAP?

\ A word can be saved by deciding if a year isn't a leap year
: -LEAP  ( year -- flag )  \ nonzero flag means not a leap year
   DUP 100 MOD 0=  OVER 400 MOD AND  SWAP 4 MOD OR ;

\ Using EXIT skips some tests for most years
: LEAPX  ( year -- flag )
   DUP 4 MOD IF DROP FALSE EXIT THEN
   DUP 100 MOD SWAP 400 MOD 0= OR ;

: -LEAPX  ( year -- flag )
   DUP 4 MOD IF EXIT THEN
   DUP 100 MOD 0= SWAP 400 MOD AND ;

2000 -LEAPX . CR
