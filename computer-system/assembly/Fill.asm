(LOOP_KBD)  // キーボードを監視
	@KBD
	D=M
	@SET_BLACK
	D; JGT
	@SET_WHITE
	0; JMP
(SET_BLACK) // スクリーンに黒を設定
	@color
	M=-1 // 16ビットのワードに全て1をセットするため-1を指定
	@SET_FILL
	0; JMP
(SET_WHITE) // スクリーンに白を設定
	@color
	M=0 
	@SET_FILL
	0; JMP
(SET_FILL)    // ループの変数をセット
	// スクリーンのアドレスをposにセット(初期化)
	@SCREEN
	D=A
	@pos
	M=D
	// ループカウンタの初期化
	@i
	M=0
(LOOP_FILL) 
	// 終端判定	
	@i
	D=M	
	// 本来であれば(32ワード * 256行)で8192回ループすべきだがシミュレータの時間が掛かるため32にする
	@32	
	D=D-A	
	@END_FILL	
	D;JGE

	// スクリーンに描画
	@color
	D=M
	@pos
	A=M
	M=D

	// カウンタのインクリメント
	@i
	M=M+1
	@pos
	M=M+1
	@LOOP_FILL
	0; JMP
(END_FILL)
	@LOOP_KBD
	0; JMP 
