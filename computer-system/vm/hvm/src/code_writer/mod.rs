use crate::parser::Command;
use crate::parser::CommandType;

pub fn write_to_file(file_name: &str, command_list: Vec<Command>) {}
pub fn write_to_stdout(command_list: Vec<Command>) {
    for command in command_list {
        let code = match command.command_type() {
            CommandType::PUSH => { 
                let index: i32 = command.arg2().parse().unwrap();
                compile_push(&command.arg1(), index)
            },
            CommandType::ARITHMETIC => compile_add(),
            _ => "".to_string(),
        };
        println!("{}", code);
    }
}

fn compile(command: Command) -> String {
    return "".to_string();
}

fn compile_arithmetic(command: Command) -> String {
    return "".to_string();
}

fn compile_push(segment: &str, index: i32) -> String {
    match segment {
        "constant" => format!("@{}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1", index),
        _ => "".to_string(),
    }
}

fn compile_add() -> String {
    "@SP\nM=M-1\nA=M\nD=M\nA=A-1\nM=M+D".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod compile_push {
        use super::*;
        #[test]
        fn test_compile_push__constant() {
            assert_eq!(
                compile_push(&"constant", 7),
                "@7\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1"
            );
        }
    }

    mod add {
        use super::*;
        #[test]
        fn test_add() {
            assert_eq!(compile_add(), "@SP\nM=M-1\nA=M\nD=M\nA=A-1\nM=M+D");
        }
    }
}
