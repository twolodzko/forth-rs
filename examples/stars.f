\ The code example from "Starting Forth" book

: STAR 42 EMIT ;
: STARS   0 DO  STAR  LOOP ;
: MARGIN  CR 30 SPACES ;
: BLIP MARGIN STAR ;
: BAR  MARGIN 5 STARS ;
: F    BAR BLIP BAR BLIP BLIP CR ;

\ Large letter F
BAR BLIP BAR BLIP BLIP  CR
