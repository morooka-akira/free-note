// Computers r2=r0 * r1
	// iの初期化(0)
	@i
	M=0
	// 計算結果(R2)の初期化(0)
	@R2 
	M=0
(LOOP)
	// ループｎ判定
	@i
	D=M
	@R1
	D=D-M
	@END
	D;JGE
	// R0を結果(R2)に加算
	@R0
	D=M // R2をDレジスタに格納しておく
	@R2
	M=D+M // R2 = R2 + R0
	// iのインクリメント
	@i
	M=M+1
	// Loop
	@LOOP
	0;JMP
(END)
	@END
	0;JMP
	
