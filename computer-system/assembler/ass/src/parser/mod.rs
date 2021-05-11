use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum CommandType {
    A,
    C,
    L,
}

#[derive(Debug)]
pub struct Command<'a> {
    raw: &'a str,
    next: Option<&'a Command<'a>>,
}

impl<'a> Command<'a> {
    pub fn command_type(&self) -> CommandType {
        // NOTE: 現時点ではシンボルが未実装のためAコマンドでないならCコマンドになる
        if self.raw.starts_with("@") {
            CommandType::A
        } else {
            CommandType::C
        }
    }

    pub fn symbol(&self) -> &str {
        if self.command_type() == CommandType::A {
            let t: Vec<&str> = self.raw.split('@').collect();
            t[1]
        } else {
            ""
        }
    }
}

pub struct Parser<'a> {
    command: &'a Command<'a>,
}

pub fn parse(filename: &str) -> Result<(), Box<dyn Error>> {
    println!("call parse {}", filename);

    let mut f = File::open(filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in contents.lines() {
        println!("{}", line);
    }

    Ok(())
}

fn parse_line(line: &str) -> Option<Command> {
    let sanitized = line.trim();
    if sanitized.starts_with("//") {
      return None;
    } 
    Some(Command { raw: sanitized.clone(), next: None })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let query = "  command str ";
        let res = parse_line(&query);
        assert!(res.is_some());
        let command = res.unwrap();
        assert_eq!(
            command.raw,
            "command str"
        );
        assert!(
            command.next.is_none(),
        )
    }

    #[test]
    fn test_parse_line_comment() {
        let query = "// some comment";
        let res = parse_line(&query);
        assert!(res.is_none());
    }

    #[test]
    fn test_command_type_for_a() {
        let query = "@100";
        let res = parse_line(&query);
        let command = res.unwrap();
        assert_eq!(
            command.command_type(),
            CommandType::A
        )
    }

    #[test]
    fn test_symbol() {
        let query = "@100";
        let res = parse_line(&query);
        let command = res.unwrap();
        assert_eq!(
            command.symbol(),
            "100"
        )
    }
}
