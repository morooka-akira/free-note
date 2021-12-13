#ifdef __APPLE__ // Macの場合は、関数名の先頭にアンダースコアが必要
	#define SET_CONTEXT _set_context
	#define SWITCH_CONTEXT _switch_context
#else
	#define SET_CONTEXT set_context
	#define SWITCH_CONTEXT switch_context
#endif

.global SET_CONTEXT
.global SWITCH_CONTEXT

SET_CONTEXT:
	// callee保存レジスタを保存
	stp d8, d9, [x0] // x0レジスタにRegisters構造体へのアドレスが格納されている
	stp d10, d11, [x0, #16] // stpは2つのレジスタの値を保存できる
	stp d12, d13, [x0, #16 * 2]
	stp d14, d15, [x0, #16 * 3]
	stp d19, d20, [x0, #16 * 4]
	stp d21, d22, [x0, #16 * 5]
	stp d23, d24, [x0, #16 * 6]
	stp d25, d26, [x0, #16 * 7]
	stp d27, d28, [x0, #16 * 8]

	// スタックポインタとリンクレジスタを保存
	mov x1, sp
	stp x30, x1, [0x, #16 * 9]

	// return 0
	mov x0, 0
	ret

SWITCH_CONTEXT:
	// callee保存レジスタを復元
	ldp d8, d9, [x0]
	ldp d10, d11, [x0, #16]
	ldp d12, d13, [x0, #16 * 2]
	ldp d14, d15, [x0, #16 * 3]
	ldp d19, d20, [x0, #16 * 4]
	ldp d21, d22, [x0, #16 * 5]
	ldp d23, d24, [x0, #16 * 6]
	ldp d25, d26, [x0, #16 * 7]
	ldp d27, d28, [x0, #16 * 8]

	// スタックポインタとリンクレジスタを復元
	ldp x30, x2, [x0, #16 * 9]
	mov sp, x2

	// return 1
	mov x0, 1
	ret