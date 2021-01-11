load Mux.hdl,
output-file Mux.out,
output-list out;

// --------------(a選択)----------------
// a: 0, b: 0, sel: 0 => 0
set a 0, set b 0, set sel 0,
eval, output;

// a: 0, b: 1, sel: 0 => 0
set a 0, set b 1, set sel 0,
eval, output;

// a: 1, b: 0, sel: 0 => 1
set a 1, set b 0, set sel 0,
eval, output;

// a: 1, b: 1, sel: 0 => 1
set a 1, set b 1, set sel 0,
eval, output;

// --------------(b選択)----------------
// a: 0, b: 0, sel: 1 => 0
set a 0, set b 0, set sel 1,
eval, output;

// a: 0, b: 1, sel: 1 => 1
set a 0, set b 1, set sel 1,
eval, output;

// a: 1, b: 0, sel: 1 => 0
set a 1, set b 0, set sel 1,
eval, output;

// a: 1, b: 1, sel: 1 => 1
set a 1, set b 1, set sel 1,
eval, output;
