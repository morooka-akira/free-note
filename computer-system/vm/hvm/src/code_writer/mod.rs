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
                compile_label("null", &sym)
            }
            CommandType::IF => compile_if("null", &command.arg1()),
            CommandType::GOTO => compile_goto("null", &command.arg1()),
            CommandType::FUNCTION => {
                let arg_cnt: i32 = command.arg2().parse().unwrap();
                compile_function(&command.arg1(), arg_cnt)
            }
            _ => [].to_vec(),
        };
        process(code_list.join("\n"));
    }
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
}
