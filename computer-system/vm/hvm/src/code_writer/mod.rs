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
        "constant" => vec![
            format!("@{}", index),
            "D=A".to_string(),
            "@SP".to_string(),
            "A=M".to_string(),
            "M=D".to_string(),
            "@SP".to_string(),
            "M=M+1".to_string(),
        ],
        _ => [].to_vec(),
    }
}

fn compile_add<'a>() -> Vec<String> {
    vec![
        "@SP".to_string(),
        "M=M-1".to_string(),
        "A=M".to_string(),
        "D=M".to_string(),
        "A=A-1".to_string(),
        "M=M+D".to_string(),
    ]
}

fn compile_sub() -> Vec<String> {
    vec![
        "@SP".to_string(),
        "M=M-1".to_string(),
        "A=M".to_string(),
        "D=M".to_string(),
        "A=A-1".to_string(),
        "M=M-D".to_string(),
    ]
}

fn compile_neg() -> Vec<String> {
    vec![
        "@SP".to_string(),
        "M=M-1".to_string(),
        "A=M".to_string(),
        "D=-D".to_string(),
    ]
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
    vec![
        // 1.yを取り出す
        "@SP".to_string(),
        "M=M-1".to_string(),
        "A=M".to_string(),
        // 最後の値(y)をDレジスタに保持
        "D=M".to_string(),
        // 2.xを取り出す(アドレスy-1)
        "A=A-1".to_string(),
        // 3.x - yの結果をDレジスタに
        "D=M-D".to_string(),
        // 4.先にTRUE(-1)をスタックに格納しておく
        // ※ Falseの場合は後で上書き
        "@SP".to_string(),
        "A=M".to_string(),
        "M=-1".to_string(),
        // 5.ジャンプ判定
        format!("@{}", skip_label).to_string(),
        format!("D;{}", jump).to_string(),
        // 6.FALSE(0)をスタックにセット
        // 5の判定でジャンプしない場合はFALSEとして値が上書きされる
        "@SP".to_string(),
        "A=M".to_string(),
        "M=0".to_string(),
        // 7.ジャンプ先のラベル
        format!("({})", skip_label).to_string(),
        // 8.スタックポインタを1進めて終了
        "@SP".to_string(),
        "M=M+1".to_string(),
    ]
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
    }

    mod arithmetic {
        use super::*;
        #[test]
        fn test_add() {
            assert_eq!(
                compile_add(),
                ["@SP", "M=M-1", "A=M", "D=M", "A=A-1", "M=M+D"]
            );
        }

        #[test]
        fn test_sub() {
            assert_eq!(
                compile_sub(),
                ["@SP", "M=M-1", "A=M", "D=M", "A=A-1", "M=M-D"]
            );
        }

        #[test]
        fn test_neg() {
            assert_eq!(compile_neg(), ["@SP", "M=M-1", "A=M", "D=-D"]);
        }

        #[test]
        fn test_eq() {
            assert_eq!(
                compile_eq(&"SKIP"),
                [
                    "@SP", "M=M-1", "A=M", "D=M", "A=A-1", "D=M-D", "@SP", "A=M", "M=-1", "@SKIP",
                    "D;JEQ", "@SP", "A=M", "M=0", "(SKIP)", "@SP", "M=M+1",
                ]
            )
        }

        #[test]
        fn test_gt() {
            assert_eq!(
                compile_gt(&"SKIP"),
                [
                    "@SP", "M=M-1", "A=M", "D=M", "A=A-1", "D=M-D", "@SP", "A=M", "M=-1", "@SKIP",
                    "D;JGT", "@SP", "A=M", "M=0", "(SKIP)", "@SP", "M=M+1",
                ]
            )
        }

        #[test]
        fn test_lt() {
            assert_eq!(
                compile_lt(&"SKIP"),
                [
                    "@SP", "M=M-1", "A=M", "D=M", "A=A-1", "D=M-D", "@SP", "A=M", "M=-1", "@SKIP",
                    "D;JLT", "@SP", "A=M", "M=0", "(SKIP)", "@SP", "M=M+1",
                ]
            )
        }
    }
}
