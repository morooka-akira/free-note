load Or16.hdl,
output-file Or16.out,
output-list out%B1.16.1;

set a %B0000000000000000,
set b %B0000000000000000,
eval, 
output;

set a %B0101010101010101,
set b %B1010101010101010,
eval, 
output;

set a %B1111111111111111,
set b %B1111111111111111,
eval, 
output;
