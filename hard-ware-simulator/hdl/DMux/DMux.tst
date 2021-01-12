load DMux.hdl,
output-file DMux.out,
output-list a b;

// in: 0, sel: 0 => a: 0, b: 0
set in 0, set sel 0,
eval, output;

// in: 0, sel: 1 => a: 0, b: 0
set in 0, set sel 1,
eval, output;

// in: 1, sel: 0 => a: 1, b: 0
set in 1, set sel 0,
eval, output;

// in: 1, sel: 1 => a: 0, b: 1
set in 1, set sel 1,
eval, output;
