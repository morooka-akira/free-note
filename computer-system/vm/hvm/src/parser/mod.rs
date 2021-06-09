use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum CommandType {
    ARITHMETIC,
    PUSH,
    POP,
    LABEL,
    GOTO,
    IF,
    FUNCTION,
    RETURN,
    CALL,
    UNKNOWN,
}

#[derive(Debug, Clone)]
pub struct Command {
    raw: String,
}

impl Command {
    pub fn command_type(&self) -> CommandType {
        let mut words = self.raw.split(" ");
        let command = words.next().unwrap();
        return match command {
            "push" => CommandType::PUSH,
            _ => CommandType::UNKNOWN,
        }
    }
}

pub fn parse(filename: &str) -> Result<Vec<Command>, Box<dyn Error>> {
    let mut f = File::open(filename)?;

    let mut commands: Vec<Command> = Vec::new();
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in contents.lines() {
        let pl = parse_line(&line);
        if let Some(c) = pl {
            commands.push(c.clone());
        }
    }
    Ok(commands)
}

/// ファイルの１行をパースしてコマンドに変換する
pub fn parse_line(line: &str) -> Option<Command> {
    let trim_reg = Regex::new(r"\s+").unwrap().replace_all(line, " ");
    let trim = trim_reg.trim();
    // コメント/空行は除外
    if trim.starts_with("//") || trim == ""  {
      return None;
    } 
    // 後方コメントも除外
    let comment = Regex::new(r"//.*").unwrap().replace(&trim, "");
    let sanitized = comment;
    Some(Command { raw: sanitized.to_string() })
}

#[cfg(test)]
mod tests {
    use super::*;

    mod command_type {
        use super::*;

        #[test]
        fn test_push() {
            assert_eq!(
                parse_line(&"push constant 7").unwrap().command_type(),
                CommandType::PUSH
            );
        }
    }

    mod parse_line {
        use super::*;

        #[test]
        fn test_include_empty() {
            assert_eq!(
                parse_line(&"  push constant 7    ").unwrap().raw,
                "push constant 7"
            );
        }

        #[test]
        fn test_comment() {
            let res = parse_line(&"// some comment");
            assert!(res.is_none());
        }

        #[test]
        fn test_white_space() {
            assert_eq!(
                parse_line(&"push       constant    7").unwrap().raw,
                "push constant 7"
            );
        }
    }
}
