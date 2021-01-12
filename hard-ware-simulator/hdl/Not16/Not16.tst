load Not16.hdl,
output-file Not16.out,
output-list out%B1.16.1;

set in %B0000000000000000,
eval, 
output;

set in %B0101010101010101,
eval, 
output;

set in %B1111111111111111,
eval, 
output;
