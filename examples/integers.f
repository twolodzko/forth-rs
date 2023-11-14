include include/std.f

\ Examples from: http://www.murphywong.net/hello/simple.htm#L15

: INTEGERS1     ( +n -- )
   1            \ Initialize i to 1     ( +n i=1 )
   BEGIN        \ Start loop: i is TOS  ( +n i )
      2DUP      \ Duplicate 2 items     ( +n i +n i )
      <         \ Is +n less than i ?   ( +n i flag )
      IF        \ Act on flag           ( +n i )
         2DROP  \ True: drop 2 items    (  )
         EXIT   \ True: leave word      (  )
      THEN      \ End IF ... THEN       (  )
      DUP       \ DUPlicate TOS         ( +n i i )
      .         \ Display TOS           ( +n i )
      1+        \ Increment TOS         ( +n i=i+1 )
   AGAIN        \ Loop back             ( +n i )
   ;            \ End definition

10 INTEGERS1

: INTEGERS2 ( +n -- )
   1        \ Initialize i to 1      ( +n i=1 )
   BEGIN    \ Start loop: i is TOS   ( +n i )
      DUP   \ DUPlicate TOS          ( +n i i )
      .     \ Display TOS            ( +n i )
      1+    \ Increment TOS          ( +n i=i+1 )
      2DUP  \ Duplicate 2 items      ( +n i +n i )
      <     \ Is +n less than i ?    ( +n i flag )
   UNTIL    \ Loop back unless true  ( +n i )
   2DROP    \ Drop two items         (  )
   ;        \ End definition

10 INTEGERS2

: INTEGERS3 ( +n -- )
   1        \ Initialize i to 1        ( +n i=1 )
   BEGIN    \ Start loop: i is TOS     ( +n i )
      2DUP  \ Duplicate 2 items        ( +n i +n i )
      < 0=  \ Is +n not less than i ?  ( +n i flag )
   WHILE    \ If true, continue else
            \ jump to after REPEAT     ( +n i )
      DUP   \ DUPlicate TOS            ( +n i i )
      .     \ Display TOS              ( +n i )
      1+    \ Increment TOS            ( +n i=i+1 )
   REPEAT   \ Loop back                ( +n i )
   2DROP    \ Drop two items           (  )
   ;        \ End definition

10 INTEGERS3
