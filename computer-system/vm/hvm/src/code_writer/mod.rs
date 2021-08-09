use crate::config::Config;
use crate::parser::Command;
use crate::parser::CommandType;
use crate::symbol_table::SymbolTable;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub fn write_to_file(command_list: Vec<Command>, config: &Config) {
    if let Some(path) = &config.output {
        let mut file = BufWriter::new(File::create(&path).unwrap());
        let mut write_process = |com| write!(file, "{}\n", com).unwrap();
        write_command(command_list, config, &mut write_process);
    }
}

pub fn write_to_stdout(command_list: Vec<Command>, config: &Config) {
    let mut write_process = |com| println!("{}", com);
    write_command(command_list, config, &mut write_process);
}

fn write_command<F>(command_list: Vec<Command>, config: &Config, process: &mut F)
where
    F: FnMut(String),
{
    let mut table = SymbolTable::new();
    let path = Path::new(&config.filename);
    let name = path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .split('.')
        .next();
    let mut fun_name: String = "null".to_string();
    let code = compile_bootstrap();
    process(code.join("\n"));
    let mut call_cnt: i32 = 0;
    for command in command_list {
        let code_list = match command.command_type() {
            CommandType::PUSH => {
                let index: i32 = command.arg2().parse().unwrap();
                compile_push(&command.arg1(), index, &name.unwrap())
            }
            CommandType::POP => {
                let index: i32 = command.arg2().parse().unwrap();
                compile_pop(&command.arg1(), index, &name.unwrap())
            }
            CommandType::ARITHMETIC => {
                let sym = command.arg1();
                compile_arithmetic(&sym, &mut table)
            }
            CommandType::LABEL => {
                let sym = command.arg1();
                compile_label(&fun_name, &sym)
            }
            CommandType::IF => compile_if(&fun_name, &command.arg1()),
            CommandType::GOTO => compile_goto(&fun_name, &command.arg1()),
            CommandType::CALL => {
                let fun: String = command.arg1();
                fun_name = fun.to_string();
                let arg_cnt: i32 = command.arg2().parse().unwrap();
                call_cnt += 1;
                compile_call(&fun, arg_cnt, call_cnt)
            }
            CommandType::FUNCTION => {
                let fun: String = command.arg1();
                fun_name = fun.to_string();
                let arg_cnt: i32 = command.arg2().parse().unwrap();
                compile_function(&fun, arg_cnt)
            }
            CommandType::RETURN => compile_return(),
            _ => [].to_vec(),
        };
        process(code_list.join("\n"));
    }
}

// // ブートストラップコード
fn compile_bootstrap() -> Vec<String> {
    let mut commands = vec![];
    // 1.SPをRAM[256]に設定する
    commands.push("@256".to_string());
    commands.push("D=A".to_string());
    commands.push("@SP".to_string());
    commands.push("M=A".to_string());
    // 2. Sys.initを呼び出す
    commands.append(&mut compile_call(&"Sys.init", 0, 0));
    return commands;
}

fn compile_arithmetic(symbol: &str, table: &mut SymbolTable) -> Vec<String> {
    match symbol {
        "add" => compile_add(),
        "sub" => compile_sub(),
        "neg" => compile_neg(),
        "or" => compile_or(),
        "and" => compile_and(),
        "not" => compile_not(),
        "eq" => {
            table.next_label();
            compile_eq(&format!("SKIP{}", table.label_count))
        }
        "gt" => {
            table.next_label();
            compile_gt(&format!("SKIP{}", table.label_count))
        }
        "lt" => {
            table.next_label();
            compile_lt(&format!("SKIP{}", table.label_count))
        }
        _ => [].to_vec(),
    }
}

fn compile_label(fun_name: &str, symbol: &str) -> Vec<String> {
    [format!("({}${})", fun_name, symbol)].to_vec()
}

fn compile_if(fun_name: &str, symbol: &str) -> Vec<String> {
    let mut commands = vec![];
    // 1.スタック最上の値を取り出す
    commands.append(&mut command_pop_to_a());
    // 2.値をDに退避
    commands.push("D=M".to_string());
    // 3.ジャンプ命令
    commands.push(format!("@{}${}", fun_name, symbol));
    commands.push("D;JNE".to_string());
    return commands;
}

fn compile_goto(fun_name: &str, symbol: &str) -> Vec<String> {
    let mut commands = vec![];
    // ジャンプ命令
    commands.push(format!("@{}${}", fun_name, symbol));
    commands.push("0;JMP".to_string());
    return commands;
}

// call_cnt: 同関数が複数回呼ばれた時のために関数全体のコール回数をシンボルに付与する
fn compile_call(fun_name: &str, arg_cnt: i32, call_cnt: i32) -> Vec<String> {
    let mut commands = vec![];
    let ret_address = format!("CALL${}${}", fun_name, call_cnt);

    // リターンアドレスをスタックに格納する
    commands.append(&mut compile_push_symbol(&ret_address));
    // LCL, ARG, THIS, THAT　を退避
    commands.append(&mut compile_push_symbol("LCL"));
    commands.append(&mut compile_push_symbol("ARG"));
    commands.append(&mut compile_push_symbol("THIS"));
    commands.append(&mut compile_push_symbol("THAT"));
    // ARGを変更
    // NOTE: callはargsをスタックに詰めたあと実行されるため今退避した5つのシンボル
    // (return address, LCL, ARG, THIS, THAT)
    // プラス 引数の数を今のSPから引いたアドレスがARGになる
    commands.push("@SP".to_string());
    commands.push("D=M".to_string());
    commands.push("@5".to_string());
    commands.push("D=D-A".to_string());
    commands.push(format!("@{}", arg_cnt));
    commands.push("D=D-A".to_string());
    commands.push("@ARG".to_string());
    commands.push("M=D".to_string());
    // LCL(ローカル変数)を現在のSPにする
    commands.push("@SP".to_string());
    commands.push("D=M".to_string());
    commands.push("@LCL".to_string());
    commands.push("M=D".to_string());
    // 関数のラベルにジャンプ
    commands.push(format!("@{}", fun_name));
    commands.push("0;JMP".to_string());
    // 関数からのリターンのためにリターンアドレスのラベルを宣言
    commands.push(format!("({})", ret_address));
    return commands;
}

fn compile_function(fun_name: &str, arg_cnt: i32) -> Vec<String> {
    let mut commands = vec![];
    // 変数名のラベルを宣言する
    commands.push(format!("({})", fun_name).to_string());
    // 変数の数だけローカルのメモリを0初期化する
    for n in 0..arg_cnt {
        // スタックに0を追加
        commands.append(&mut compile_push_constants(0));
        // ローカル変数を0初期化
        commands.append(&mut compile_pop_local(n));
    }
    return commands;
}

fn compile_return() -> Vec<String> {
    let mut commands = vec![];
    // 1.LCLは関数フレームの先頭のためアドレス起点として保持しておく(R13)
    commands.push("@LCL".to_string());
    commands.push("D=M".to_string());
    commands.push("@R13".to_string());
    commands.push("M=D".to_string());
    // 2.リターンアドレス(LCL - 5)の取得
    // R14に保持
    commands.push("@5".to_string());
    commands.push("D=A".to_string());
    commands.push("@R13".to_string());
    commands.push("D=M-D".to_string());
    commands.push("A=D".to_string());
    commands.push("D=M".to_string());
    commands.push("@R14".to_string());
    commands.push("M=D".to_string());

    // 3.戻り値の格納
    // スタックTOPの値をARGに入れる(ARGがスタックの最後の値に復元されるため)
    commands.append(&mut command_pop_to_a());
    commands.push("D=M".to_string());
    commands.push("@ARG".to_string());
    commands.push("A=M".to_string());
    commands.push("M=D".to_string());

    // 3.SPの復元
    // ARGに戻り値を入れたのでSPは+1のアドレスになる
    commands.push("A=A+1".to_string());
    commands.push("D=A".to_string());
    commands.push("@SP".to_string());
    commands.push("M=D".to_string());

    // 4. THAT, THIS, ARG, LCLの復元
    // LCLの上にはTHAT, THIS, ARG, LCLの順番で呼び出し前の値が保存されているため復元する
    commands.append(&mut command_restore_segment("R13", 1, "THAT"));

    commands.append(&mut command_restore_segment("R13", 2, "THIS"));

    commands.append(&mut command_restore_segment("R13", 3, "ARG"));

    commands.append(&mut command_restore_segment("R13", 4, "LCL"));

    // 5. リターンアドレスにジャンプ
    commands.push("@R14".to_string());
    commands.push("A=M".to_string());
    commands.push("0;JMP".to_string());
    return commands;
}

// baseシンボルから-indexした値でsegmentの指定アドレスを書き換える
// アドレスの復元
fn command_restore_segment(base_symbol: &str, index: i32, segment_symbol: &str) -> Vec<String> {
    vec![
        format!("@{}", base_symbol),
        "D=M".to_string(),
        format!("@{}", index),
        "D=D-A".to_string(),
        "A=D".to_string(),
        "D=M".to_string(),
        format!("@{}", segment_symbol),
        "M=D".to_string(),
    ]
}

fn compile_push(segment: &str, index: i32, filename: &str) -> Vec<String> {
    match segment {
        "constant" => compile_push_constants(index),
        "local" => compile_push_local(index),
        "argument" => compile_push_argument(index),
        "this" => compile_push_this(index),
        "that" => compile_push_that(index),
        "temp" => compile_push_temp(index),
        "pointer" => compile_push_pointer(index),
        "static" => compile_push_static(index, filename),
        _ => [].to_vec(),
    }
}

fn compile_pop(segment: &str, index: i32, filename: &str) -> Vec<String> {
    match segment {
        "local" => compile_pop_local(index),
        "argument" => compile_pop_argument(index),
        "this" => compile_pop_this(index),
        "that" => compile_pop_that(index),
        "temp" => compile_pop_temp(index),
        "pointer" => compile_pop_pointer(index),
        "static" => compile_pop_static(index, filename),
        _ => [].to_vec(),
    }
}

/// スタックポインタが指す値を一つ取り出しAレジスタに入れる
fn command_pop_to_a() -> Vec<String> {
    vec!["@SP".to_string(), "M=M-1".to_string(), "A=M".to_string()]
}

// スタックポインタを進める
fn command_increment_sp() -> Vec<String> {
    vec!["@SP".to_string(), "M=M+1".to_string()]
}

// 指定のセグメントのアドレスをAレジスタ格納する
fn command_segment_to_a(symbol: &str, index: i32) -> Vec<String> {
    vec![
        format!("@{}", index),
        "D=A".to_string(),
        format!("@{}", symbol),
        "A=D+M".to_string(),
    ]
}

// 指定のレジスタ番号のアドレスをAレジスタ格納する
fn command_register_to_a(register_number: i32) -> Vec<String> {
    vec![format!("@R{}", register_number)]
}

/* --------------------------- push ------------------------------- */

fn compile_push_constants(index: i32) -> Vec<String> {
    let mut commands = vec![
        format!("@{}", index),
        "D=A".to_string(),
        "@SP".to_string(),
        "A=M".to_string(),
        "M=D".to_string(),
    ];
    // 3.スタックのポインタをインクリメント
    commands.append(&mut command_increment_sp());
    return commands;
}

// シンボルのアドレスをスタックに退避する
fn compile_push_symbol(symbol: &str) -> Vec<String> {
    let mut commands = vec![
        format!("@{}", symbol),
        "D=A".to_string(),
        "@SP".to_string(),
        "A=M".to_string(),
        "M=D".to_string(),
    ];
    // 3.スタックのポインタをインクリメント
    commands.append(&mut command_increment_sp());
    return commands;
}

fn compile_push_register(register_number: i32) -> Vec<String> {
    return compile_push_with_commands(&mut command_register_to_a(register_number));
}

fn compile_push_segment(symbol: &str, index: i32) -> Vec<String> {
    return compile_push_with_commands(&mut command_segment_to_a(symbol, index));
}

fn compile_push_with_commands(target_commands: &mut Vec<String>) -> Vec<String> {
    // 1.ローカル変数の値(base + index)を取り出す
    let mut commands = vec![];
    commands.append(target_commands);
    commands.append(&mut vec![
        "D=M".to_string(),
        // 2.スタックに格納
        "@SP".to_string(),
        "A=M".to_string(),
        "M=D".to_string(),
    ]);
    // 3.スタックのポインタをインクリメント
    commands.append(&mut command_increment_sp());
    return commands;
}

fn compile_push_local(index: i32) -> Vec<String> {
    compile_push_segment("LCL", index)
}

fn compile_push_argument(index: i32) -> Vec<String> {
    compile_push_segment("ARG", index)
}

fn compile_push_this(index: i32) -> Vec<String> {
    compile_push_segment("THIS", index)
}

fn compile_push_that(index: i32) -> Vec<String> {
    compile_push_segment("THAT", index)
}

fn compile_push_pointer(index: i32) -> Vec<String> {
    compile_push_register(3 + index)
}

fn compile_push_temp(index: i32) -> Vec<String> {
    compile_push_register(5 + index)
}

fn compile_push_static(index: i32, filename: &str) -> Vec<String> {
    return compile_push_with_commands(&mut vec![format!("@{}.{}", filename, index)]);
}

/* --------------------------- pop ------------------------------- */

fn compile_pop_with_commands(target_commands: &mut Vec<String>) -> Vec<String> {
    // 1.ローカル変数の値(base + index)を取り出す
    let mut commands = vec![];
    commands.append(target_commands);
    // AレジスタのアドレスをR13に退避する
    commands.append(&mut vec![
        "D=A".to_string(),
        "@R13".to_string(),
        "M=D".to_string(),
    ]);
    // 2.スタック最上の値を取り出す
    commands.append(&mut command_pop_to_a());
    // 3.値をDに退避
    commands.push("D=M".to_string());
    // 4. R13のアドレスの指定先にスタック上部の値を格納
    commands.push("@R13".to_string());
    commands.push("A=M".to_string());
    commands.push("M=D".to_string());
    return commands;
}

fn compile_pop_segment(symbol: &str, index: i32) -> Vec<String> {
    return compile_pop_with_commands(&mut command_segment_to_a(symbol, index));
}

fn compile_pop_register(register_number: i32) -> Vec<String> {
    return compile_pop_with_commands(&mut command_register_to_a(register_number));
}

fn compile_pop_local(index: i32) -> Vec<String> {
    compile_pop_segment("LCL", index)
}

fn compile_pop_argument(index: i32) -> Vec<String> {
    compile_pop_segment("ARG", index)
}

fn compile_pop_this(index: i32) -> Vec<String> {
    compile_pop_segment("THIS", index)
}

fn compile_pop_that(index: i32) -> Vec<String> {
    compile_pop_segment("THAT", index)
}

fn compile_pop_pointer(index: i32) -> Vec<String> {
    compile_pop_register(3 + index)
}

fn compile_pop_static(index: i32, filename: &str) -> Vec<String> {
    return compile_pop_with_commands(&mut vec![format!("@{}.{}", filename, index)]);
}

fn compile_pop_temp(index: i32) -> Vec<String> {
    compile_pop_register(5 + index)
}

fn compile_add() -> Vec<String> {
    let mut commands = vec![];
    // 1.yを取り出す
    commands.append(&mut command_pop_to_a());
    // 2.yをDに退避
    commands.push("D=M".to_string());
    // 3.xを取り出す
    commands.append(&mut command_pop_to_a());
    // 4. D = x + y
    commands.push("M=M+D".to_string());
    // 5.スタックポインタを1進めて終了
    commands.append(&mut command_increment_sp());
    return commands;
}

fn compile_sub() -> Vec<String> {
    let mut commands = vec![];
    // 1.yを取り出す
    commands.append(&mut command_pop_to_a());
    // 2.yをDに退避
    commands.push("D=M".to_string());
    // 3.xを取り出す
    commands.append(&mut command_pop_to_a());
    // 4. D = x - y
    commands.push("M=M-D".to_string());
    // 5.スタックポインタを1進めて終了
    commands.append(&mut command_increment_sp());
    return commands;
}

fn compile_neg() -> Vec<String> {
    let mut commands = vec![];
    // 1.yを取り出す
    commands.append(&mut command_pop_to_a());
    // 2. M = - M
    commands.push("M=-M".to_string());
    // 3.スタックポインタを1進めて終了
    commands.append(&mut command_increment_sp());
    return commands;
}

fn compile_or() -> Vec<String> {
    let mut commands = vec![];
    // 1.yを取り出す
    commands.append(&mut command_pop_to_a());
    // 2.yをDに退避
    commands.push("D=M".to_string());
    // 3.xを取り出す
    commands.append(&mut command_pop_to_a());
    // 4. D = x | Y
    commands.push("M=D|M".to_string());
    // 5.スタックポインタを1進めて終了
    commands.append(&mut command_increment_sp());
    return commands;
}

fn compile_and() -> Vec<String> {
    let mut commands = vec![];
    // 1.yを取り出す
    commands.append(&mut command_pop_to_a());
    // 2.yをDに退避
    commands.push("D=M".to_string());
    // 3.xを取り出す
    commands.append(&mut command_pop_to_a());
    // 4. D = x & Y
    commands.push("M=D&M".to_string());
    // 5.スタックポインタを1進めて終了
    commands.append(&mut command_increment_sp());
    return commands;
}

fn compile_not() -> Vec<String> {
    let mut commands = vec![];
    // 1.yを取り出す
    commands.append(&mut command_pop_to_a());
    // 2. M = not Y
    commands.push("M=!M".to_string());
    // 3.スタックポインタを1進めて終了
    commands.append(&mut command_increment_sp());
    return commands;
}

fn compile_eq(skip_label: &str) -> Vec<String> {
    compile_comp(skip_label, &"JEQ")
}

fn compile_gt(skip_label: &str) -> Vec<String> {
    compile_comp(skip_label, &"JGT")
}

fn compile_lt(skip_label: &str) -> Vec<String> {
    compile_comp(skip_label, &"JLT")
}

// 比較演算の出力
fn compile_comp(skip_label: &str, jump: &str) -> Vec<String> {
    let mut commands = vec![];
    // 1.yを取り出す
    commands.append(&mut command_pop_to_a());
    // 2.yをDに退避
    commands.push("D=M".to_string());
    // 3.xを取り出す
    commands.append(&mut command_pop_to_a());
    // 4. D = x - y
    commands.push("D=M-D".to_string());
    commands.append(&mut vec![
        // 5.先にTRUE(-1)をスタックに格納しておく
        // ※ Falseの場合は後で上書き
        "@SP".to_string(),
        "A=M".to_string(),
        "M=-1".to_string(),
        // 6.ジャンプ判定
        format!("@{}", skip_label).to_string(),
        format!("D;{}", jump).to_string(),
        // 7.FALSE(0)をスタックにセット
        // 6の判定でジャンプしない場合はFALSEとして値が上書きされる
        "@SP".to_string(),
        "A=M".to_string(),
        "M=0".to_string(),
        // 8.ジャンプ先のラベル
        format!("({})", skip_label).to_string(),
    ]);
    // 9.スタックポインタを1進めて終了
    commands.append(&mut command_increment_sp());
    return commands;
}

#[cfg(test)]
mod tests {
    use super::*;

    mod compile_push {
        use super::*;
        #[test]
        fn test_compile_push_constant() {
            assert_eq!(
                compile_push(&"constant", 7, "Test"),
                ["@7", "D=A", "@SP", "A=M", "M=D", "@SP", "M=M+1"]
            );
        }

        #[test]
        fn test_compile_push_local() {
            assert_eq!(
                compile_push(&"local", 0, "Test"),
                ["@0", "D=A", "@LCL", "A=D+M", "D=M", "@SP", "A=M", "M=D", "@SP", "M=M+1"]
            );
        }

        #[test]
        fn test_compile_push_argument() {
            assert_eq!(
                compile_push_argument(1),
                ["@1", "D=A", "@ARG", "A=D+M", "D=M", "@SP", "A=M", "M=D", "@SP", "M=M+1"]
            );
        }

        #[test]
        fn test_compile_push_this() {
            assert_eq!(
                compile_push_this(6),
                ["@6", "D=A", "@THIS", "A=D+M", "D=M", "@SP", "A=M", "M=D", "@SP", "M=M+1"]
            );
        }

        #[test]
        fn test_compile_push_that() {
            assert_eq!(
                compile_push_that(4),
                ["@4", "D=A", "@THAT", "A=D+M", "D=M", "@SP", "A=M", "M=D", "@SP", "M=M+1"]
            );
        }

        #[test]
        fn test_compile_push_temp() {
            assert_eq!(
                compile_push_temp(6),
                ["@R11", "D=M", "@SP", "A=M", "M=D", "@SP", "M=M+1"]
            );
        }

        #[test]
        fn test_compile_push_pointer() {
            assert_eq!(
                compile_push_pointer(1),
                ["@R4", "D=M", "@SP", "A=M", "M=D", "@SP", "M=M+1"]
            );
        }

        #[test]
        fn test_compile_push_static() {
            assert_eq!(
                compile_push_static(3, "Test"),
                ["@Test.3", "D=M", "@SP", "A=M", "M=D", "@SP", "M=M+1"]
            );
        }
    }

    mod compile_pop {
        use super::*;
        #[test]
        fn test_compile_pop_local() {
            assert_eq!(
                compile_pop_local(0),
                [
                    "@0", "D=A", "@LCL", "A=D+M", "D=A", "@R13", "M=D", "@SP", "M=M-1", "A=M",
                    "D=M", "@R13", "A=M", "M=D"
                ]
            );
        }

        #[test]
        fn test_compile_pop_argument() {
            assert_eq!(
                compile_pop_argument(1),
                [
                    "@1", "D=A", "@ARG", "A=D+M", "D=A", "@R13", "M=D", "@SP", "M=M-1", "A=M",
                    "D=M", "@R13", "A=M", "M=D"
                ]
            );
        }

        #[test]
        fn test_compile_pop_this() {
            assert_eq!(
                compile_pop_this(3),
                [
                    "@3", "D=A", "@THIS", "A=D+M", "D=A", "@R13", "M=D", "@SP", "M=M-1", "A=M",
                    "D=M", "@R13", "A=M", "M=D"
                ]
            );
        }

        #[test]
        fn test_compile_pop_that() {
            assert_eq!(
                compile_pop_that(4),
                [
                    "@4", "D=A", "@THAT", "A=D+M", "D=A", "@R13", "M=D", "@SP", "M=M-1", "A=M",
                    "D=M", "@R13", "A=M", "M=D"
                ]
            );
        }

        #[test]
        fn test_compile_pop_temp() {
            assert_eq!(
                compile_pop_temp(5),
                [
                    "@R10", "D=A", "@R13", "M=D", "@SP", "M=M-1", "A=M", "D=M", "@R13", "A=M",
                    "M=D"
                ]
            );
        }

        #[test]
        fn test_compile_pop_pointer() {
            assert_eq!(
                compile_pop_pointer(2),
                ["@R5", "D=A", "@R13", "M=D", "@SP", "M=M-1", "A=M", "D=M", "@R13", "A=M", "M=D"]
            );
        }

        #[test]
        fn test_compile_pop_static() {
            assert_eq!(
                compile_pop_static(1, "Test"),
                [
                    "@Test.1", "D=A", "@R13", "M=D", "@SP", "M=M-1", "A=M", "D=M", "@R13", "A=M",
                    "M=D"
                ]
            );
        }
    }

    mod arithmetic {
        use super::*;
        #[test]
        fn test_add() {
            assert_eq!(
                compile_add(),
                ["@SP", "M=M-1", "A=M", "D=M", "@SP", "M=M-1", "A=M", "M=M+D", "@SP", "M=M+1"]
            );
        }

        #[test]
        fn test_sub() {
            assert_eq!(
                compile_sub(),
                ["@SP", "M=M-1", "A=M", "D=M", "@SP", "M=M-1", "A=M", "M=M-D", "@SP", "M=M+1"]
            );
        }

        #[test]
        fn test_neg() {
            assert_eq!(
                compile_neg(),
                ["@SP", "M=M-1", "A=M", "M=-M", "@SP", "M=M+1"]
            );
        }

        #[test]
        fn test_eq() {
            assert_eq!(
                compile_eq(&"SKIP"),
                [
                    "@SP", "M=M-1", "A=M", "D=M", "@SP", "M=M-1", "A=M", "D=M-D", "@SP", "A=M",
                    "M=-1", "@SKIP", "D;JEQ", "@SP", "A=M", "M=0", "(SKIP)", "@SP", "M=M+1",
                ]
            )
        }

        #[test]
        fn test_gt() {
            assert_eq!(
                compile_gt(&"SKIP"),
                [
                    "@SP", "M=M-1", "A=M", "D=M", "@SP", "M=M-1", "A=M", "D=M-D", "@SP", "A=M",
                    "M=-1", "@SKIP", "D;JGT", "@SP", "A=M", "M=0", "(SKIP)", "@SP", "M=M+1",
                ]
            )
        }

        #[test]
        fn test_lt() {
            assert_eq!(
                compile_lt(&"SKIP"),
                [
                    "@SP", "M=M-1", "A=M", "D=M", "@SP", "M=M-1", "A=M", "D=M-D", "@SP", "A=M",
                    "M=-1", "@SKIP", "D;JLT", "@SP", "A=M", "M=0", "(SKIP)", "@SP", "M=M+1",
                ]
            )
        }

        #[test]
        fn test_or() {
            assert_eq!(
                compile_or(),
                ["@SP", "M=M-1", "A=M", "D=M", "@SP", "M=M-1", "A=M", "M=D|M", "@SP", "M=M+1",]
            )
        }

        #[test]
        fn test_and() {
            assert_eq!(
                compile_and(),
                ["@SP", "M=M-1", "A=M", "D=M", "@SP", "M=M-1", "A=M", "M=D&M", "@SP", "M=M+1",]
            )
        }

        #[test]
        fn test_not() {
            assert_eq!(
                compile_not(),
                ["@SP", "M=M-1", "A=M", "M=!M", "@SP", "M=M+1",]
            )
        }
    }

    mod compile_label {
        use super::*;
        #[test]
        fn test_no_function() {
            assert_eq!(compile_label("null", "LOOP"), ["(null$LOOP)",])
        }
    }

    mod compile_if {
        use super::*;
        #[test]
        fn test_if_goto() {
            assert_eq!(
                compile_if("null", "LOOP"),
                ["@SP", "M=M-1", "A=M", "D=M", "@null$LOOP", "D;JNE"]
            )
        }
    }

    mod compile_goto {
        use super::*;
        #[test]
        fn test_goto() {
            assert_eq!(compile_goto("null", "END"), ["@null$END", "0;JMP"])
        }
    }

    mod compile_function {
        use super::*;
        #[test]
        fn test_function() {
            assert_eq!(
                compile_function("SimpleFunction.test", 2),
                [
                    "(SimpleFunction.test)",
                    "@0",
                    "D=A",
                    "@SP",
                    "A=M",
                    "M=D",
                    "@SP",
                    "M=M+1",
                    "@0",
                    "D=A",
                    "@LCL",
                    "A=D+M",
                    "D=A",
                    "@R13",
                    "M=D",
                    "@SP",
                    "M=M-1",
                    "A=M",
                    "D=M",
                    "@R13",
                    "A=M",
                    "M=D",
                    "@0",
                    "D=A",
                    "@SP",
                    "A=M",
                    "M=D",
                    "@SP",
                    "M=M+1",
                    "@1",
                    "D=A",
                    "@LCL",
                    "A=D+M",
                    "D=A",
                    "@R13",
                    "M=D",
                    "@SP",
                    "M=M-1",
                    "A=M",
                    "D=M",
                    "@R13",
                    "A=M",
                    "M=D",
                ]
            )
        }
    }

    mod compile_return {
        use super::*;
        #[test]
        fn test_return() {
            assert_eq!(
                compile_return(),
                [
                    "@LCL", "D=M", "@R13", "M=D", "@5", "D=A", "@R13", "D=M-D", "A=D", "D=M",
                    "@R14", "M=D", "@SP", "M=M-1", "A=M", "D=M", "@ARG", "A=M", "M=D", "A=A+1",
                    "D=A", "@SP", "M=D", "@R13", "D=M", "@1", "D=D-A", "A=D", "D=M", "@THAT",
                    "M=D", "@R13", "D=M", "@2", "D=D-A", "A=D", "D=M", "@THIS", "M=D", "@R13",
                    "D=M", "@3", "D=D-A", "A=D", "D=M", "@ARG", "M=D", "@R13", "D=M", "@4",
                    "D=D-A", "A=D", "D=M", "@LCL", "M=D", "@R14", "A=M", "0;JMP"
                ]
            )
        }
    }
}
