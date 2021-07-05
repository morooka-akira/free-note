use crate::parser::Command;
use crate::parser::CommandType;
use crate::symbol_table::SymbolTable;

pub fn write_to_file(file_name: &str, command_list: Vec<Command>) {}
pub fn write_to_stdout(command_list: Vec<Command>) {
    let mut table = SymbolTable::new();
    for command in command_list {
        let code_list = match command.command_type() {
            CommandType::PUSH => {
                let index: i32 = command.arg2().parse().unwrap();
                compile_push(&command.arg1(), index)
            }
            CommandType::POP => {
                let index: i32 = command.arg2().parse().unwrap();
                compile_pop(&command.arg1(), index)
            }
            CommandType::ARITHMETIC => {
                let sym = command.arg1();
                compile_arithmetic(&sym, &mut table)
            }
            _ => [].to_vec(),
        };
        println!("{}", code_list.join("\n"));
    }
}

fn compile(command: Command) -> String {
    return "".to_string();
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

fn compile_push(segment: &str, index: i32) -> Vec<String> {
    match segment {
        "constant" => compile_push_constants(index),
        "local" => compile_push_local(index),
        _ => [].to_vec(),
    }
}

fn compile_pop(segment: &str, index: i32) -> Vec<String> {
    match segment {
        "local" => compile_push_local(index),
        _ => [].to_vec(),
    }
}

/// スタックポインタが指す値を一つ取り出しAレジスタに入れる
fn pop_to_a_commands() -> Vec<String> {
    vec!["@SP".to_string(), "M=M-1".to_string(), "A=M".to_string()]
}

fn increment_sp_commands() -> Vec<String> {
    vec!["@SP".to_string(), "M=M+1".to_string()]
}

fn compile_push_constants(index: i32) -> Vec<String> {
    let mut commands = vec![
        format!("@{}", index),
        "D=A".to_string(),
        "@SP".to_string(),
        "A=M".to_string(),
        "M=D".to_string(),
    ];
    // 3.スタックのポインタをインクリメント
    commands.append(&mut increment_sp_commands());
    return commands;
}

fn compile_push_local(index: i32) -> Vec<String> {
    let mut commands = vec![
        // 1.ローカル変数の値(base + index)を取り出す
        format!("@{}", index),
        "D=A".to_string(),
        "@LCL".to_string(),
        "A=D+M".to_string(),
        "D=M".to_string(),
        // 2.スタックに格納
        "@SP".to_string(),
        "A=M".to_string(),
        "M=D".to_string(),
    ];
    // 3.スタックのポインタをインクリメント
    commands.append(&mut increment_sp_commands());
    return commands;
}

fn compile_pop_local(index: i32) -> Vec<String> {
    let mut commands = vec![];
    // 1.ローカル変数のアドレス(base + index)をR13レジスタに退避する
    commands.append(&mut vec![
        format!("@{}", index),
        "D=A".to_string(),
        "@LCL".to_string(),
        "A=D+M".to_string(),
        "D=A".to_string(),
        "@R13".to_string(),
        "M=D".to_string()
    ]);
    // 2.スタック最上の値を取り出す
    commands.append(&mut pop_to_a_commands());
    // 3.値をDに退避
    commands.push("D=M".to_string());
    // 4. R13のアドレスの指定先にスタック上部の値を格納
    commands.push("@R13".to_string());
    commands.push("M=D".to_string());
    return commands;
}

fn compile_add() -> Vec<String> {
    let mut commands = vec![];
    // 1.yを取り出す
    commands.append(&mut pop_to_a_commands());
    // 2.yをDに退避
    commands.push("D=M".to_string());
    // 3.xを取り出す
    commands.append(&mut pop_to_a_commands());
    // 4. D = x + y
    commands.push("M=M+D".to_string());
    // 5.スタックポインタを1進めて終了
    commands.append(&mut increment_sp_commands());
    return commands;
}

fn compile_sub() -> Vec<String> {
    let mut commands = vec![];
    // 1.yを取り出す
    commands.append(&mut pop_to_a_commands());
    // 2.yをDに退避
    commands.push("D=M".to_string());
    // 3.xを取り出す
    commands.append(&mut pop_to_a_commands());
    // 4. D = x - y
    commands.push("M=M-D".to_string());
    // 5.スタックポインタを1進めて終了
    commands.append(&mut increment_sp_commands());
    return commands;
}

fn compile_neg() -> Vec<String> {
    let mut commands = vec![];
    // 1.yを取り出す
    commands.append(&mut pop_to_a_commands());
    // 2. M = - M
    commands.push("M=-M".to_string());
    // 3.スタックポインタを1進めて終了
    commands.append(&mut increment_sp_commands());
    return commands;
}

fn compile_or() -> Vec<String> {
    let mut commands = vec![];
    // 1.yを取り出す
    commands.append(&mut pop_to_a_commands());
    // 2.yをDに退避
    commands.push("D=M".to_string());
    // 3.xを取り出す
    commands.append(&mut pop_to_a_commands());
    // 4. D = x | Y
    commands.push("M=D|M".to_string());
    // 5.スタックポインタを1進めて終了
    commands.append(&mut increment_sp_commands());
    return commands;
}

fn compile_and() -> Vec<String> {
    let mut commands = vec![];
    // 1.yを取り出す
    commands.append(&mut pop_to_a_commands());
    // 2.yをDに退避
    commands.push("D=M".to_string());
    // 3.xを取り出す
    commands.append(&mut pop_to_a_commands());
    // 4. D = x & Y
    commands.push("M=D&M".to_string());
    // 5.スタックポインタを1進めて終了
    commands.append(&mut increment_sp_commands());
    return commands;
}

fn compile_not() -> Vec<String> {
    let mut commands = vec![];
    // 1.yを取り出す
    commands.append(&mut pop_to_a_commands());
    // 2. M = not Y
    commands.push("M=!M".to_string());
    // 3.スタックポインタを1進めて終了
    commands.append(&mut increment_sp_commands());
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
    commands.append(&mut pop_to_a_commands());
    // 2.yをDに退避
    commands.push("D=M".to_string());
    // 3.xを取り出す
    commands.append(&mut pop_to_a_commands());
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
    commands.append(&mut increment_sp_commands());
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
                compile_push(&"constant", 7),
                ["@7", "D=A", "@SP", "A=M", "M=D", "@SP", "M=M+1"]
            );
        }

        #[test]
        fn test_compile_push_local() {
            assert_eq!(
                compile_push(&"local", 0),
                ["@0", "D=A", "@LCL", "A=D+M", "D=M", "@SP", "A=M", "M=D", "@SP", "M=M+1"]
            );
        }
    }

    mod compile_pop {
        use super::*;
        #[test]
        fn test_compile_pop_local() {
            assert_eq!(
                compile_pop_local(0),
                ["@0", "D=A", "@LCL", "A=D+M", "D=A", "@R13", "M=D", "@SP", "M=M-1", "A=M", "D=M", "@R13", "M=D"]
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
}
