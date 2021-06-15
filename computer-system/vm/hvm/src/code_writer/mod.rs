use crate::parser::Command;
use crate::parser::CommandType;

pub fn write_to_file(file_name: &str, command_list: Vec<Command>) {}

fn compile(command: Command) -> String {
    return "".to_string()
}

fn compile_arithmetic(command: Command) -> String {
    return "".to_string()
}

fn compile_push(segment: &str, index: u32) -> String {
    return format!("@{}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1", index)
}


#[cfg(test)]
mod tests {
    use super::*;

    mod compile_push_pop {
        use super::*;
        #[test]
        fn test_compile_push__constant() {
            assert_eq!(
                compile_push(&"constant", 7),
                "@7\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1"
            );
        }
    }
}