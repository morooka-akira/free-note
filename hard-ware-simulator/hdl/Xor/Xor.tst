load Xor.hdl,
output-file Xor.out,
output-list out;

// a:0 b:0 の時
set a 0, set b 0,
eval, output;

// a:0 b:1 の時
set a 0, set b 1,
eval, output;

// a:1 b:0 の時
set a 1, set b 0,
eval, output;

// a:1 b:1 の時
set a 1, set b 1,
eval, output;
