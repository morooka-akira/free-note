load Mux4Way16.hdl,
output-file Mux4Way16.out,
output-list out%B1.16.1;

// --------------a----------------
set a %B0000000000000000,
set b %B0000000000000000,
set c %B0000000000000000,
set d %B0000000000000000,
set sel %B00,
eval, output;

set a %B0000000000000000,
set b %B1111111111111111,
set c %B1111111111111111,
set d %B1111111111111111,
set sel %B00,
eval, output;

set a %B1111111111111111,
set b %B0000000000000000,
set c %B0000000000000000,
set d %B0000000000000000,
set sel %B00,
eval, output;

// --------------b----------------
set a %B0000000000000000,
set b %B0000000000000000,
set c %B0000000000000000,
set d %B0000000000000000,
set sel %B01,
eval, output;

set a %B1111111111111111,
set b %B0000000000000000,
set c %B1111111111111111,
set d %B1111111111111111,
set sel %B01,
eval, output;

set a %B0000000000000000,
set b %B1111111111111111,
set c %B0000000000000000,
set d %B0000000000000000,
set sel %B01,
eval, output;

// --------------c----------------
set a %B0000000000000000,
set b %B0000000000000000,
set c %B0000000000000000,
set d %B0000000000000000,
set sel %B10,
eval, output;

set a %B1111111111111111,
set b %B1111111111111111,
set c %B0000000000000000,
set d %B1111111111111111,
set sel %B10,
eval, output;

set a %B0000000000000000,
set b %B0000000000000000,
set c %B1111111111111111,
set d %B0000000000000000,
set sel %B10,
eval, output;

// --------------d----------------
set a %B0000000000000000,
set b %B0000000000000000,
set c %B0000000000000000,
set d %B0000000000000000,
set sel %B11,
eval, output;

set a %B1111111111111111,
set b %B1111111111111111,
set c %B1111111111111111,
set d %B0000000000000000,
set sel %B11,
eval, output;

set a %B0000000000000000,
set b %B0000000000000000,
set c %B0000000000000000,
set d %B1111111111111111,
set sel %B11,
eval, output;
