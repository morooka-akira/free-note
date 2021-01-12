load Mux16.hdl,
output-file Mux16.out,
output-list out%B1.16.1;

set a %B0000000000000000,
set b %B0000000000000000,
set sel 0,
eval, output;

set a %B0000000000000000,
set b %B1111111111111111,
set sel 0,
eval, output;

set a %B1111111111111111,
set b %B0000000000000000,
set sel 0,
eval, output;

set a %B1111111111111111,
set b %B1111111111111111,
set sel 0,
eval, output;

set a %B0000000000000000,
set b %B0000000000000000,
set sel 1,
eval, output;

set a %B0000000000000000,
set b %B1111111111111111,
set sel 1,
eval, output;

set a %B1111111111111111,
set b %B0000000000000000,
set sel 1,
eval, output;

set a %B1111111111111111,
set b %B1111111111111111,
set sel 1,
eval, output;

