load Or.hdl,
output-file Or.out,
output-list out;

// a: 0, b: 0 => 0
set a 0, set b 0,
eval, output;

// a: 1, b: 0 => 1
set a 1, set b 0,
eval, output;

// a: 0, b: 1 => 1
set a 0, set b 1,
eval, output;

// a: 1, b: 1 => 1
set a 1, set b 1,
eval, output;
