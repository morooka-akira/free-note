use crate::parser::Command;
use crate::parser::CommandType;

pub fn write_to_file(file_name: &str, command_list: Vec<Command>) {}
pub fn write_to_stdout(command_list: Vec<Command>) {
    for command in command_list {
        let code_list = match command.command_type() {
            CommandType::PUSH => {
                let index: i32 = command.arg2().parse().unwrap();
                compile_push(&command.arg1(), index)
            }
            CommandType::ARITHMETIC => {
                let sym = command.arg1();
                compile_arithmetic(&sym)
            }
            _ => [].to_vec(),
        };
        println!("{}", code_list.join("\n"));
    }
}

fn compile(command: Command) -> String {
    return "".to_string();
}

fn compile_arithmetic(symbol: &str) -> Vec<String> {
    match symbol {
        "add" => compile_add(),
        "sub" => compile_sub(),
        "neg" => compile_neg(),
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
    }
}
