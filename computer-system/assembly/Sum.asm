// Computers sum=1+...+100
	@i		// Aレジスタにiのアドレスを入れる
	M=1		// MはAレジスタのアドレス参照先を指す(i=1)
	@sum 	// Aレジスタにsumのアドレスを入れる
	M=0		// sum=0
(LOOP)		// LOOPラベルの宣言
	@i
	D=M		// Dレジスタにiの値を入れる
	@100	// Aレジスタに100を入れる
	D=D-A	// D = D-100 (Dにはiが入っている, i-100が0寄り大きい、すなわち100まわったら終了) 
	@END	// AレジスタにENDラベルのアドレスを入れる
	D;JGT	// Dが0より大きければAレジスタのアドレスへJump(ENDにJump)
	@i		
	D=M		// D=i
	@sum
	M=D+M	// Mはsumを指す(sum=i+sum)
	@i
	M=M+1	// i++
	@LOOP
	0;JMP	// LOOPへ強制Jump
(END)		// ENDラベルの宣言
	@END
	0;JMP	// 無限ループ
