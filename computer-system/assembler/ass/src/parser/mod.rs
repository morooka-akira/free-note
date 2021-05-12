use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum CommandType {
    A,
    C,
    L,
}

#[derive(Debug, PartialEq)]
pub enum JumpType {
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
    Null
}

#[derive(Debug, PartialEq)]
pub enum DestType {
    M,
    D,
    MD,
    A,
    AM,
    AD,
    ADM,
    Null
}

#[derive(Debug, Clone)]
pub struct Command {
    raw: String,
}

impl Command {
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

    pub fn jump(&self) -> JumpType {
        if self.has_jump() {
            let a: Vec<&str> = self.raw.split(';').collect();
            match a[1] {
                "JGT" => JumpType::JGT,
                "JEQ" => JumpType::JEQ,
                "JGE" => JumpType::JGE,
                "JLT" => JumpType::JLT,
                "JNE" => JumpType::JNE,
                "JLE" => JumpType::JLE,
                "JMP" => JumpType::JMP,
                _ => JumpType::Null
            } 
        } else {
            JumpType::Null
        }
    }

    pub fn dest(&self) -> DestType {
        // NOTE: jump命令時はdestはない
        if self.has_jump() {
            DestType::Null
        } else if self.has_eq() {
            let a: Vec<&str> = self.raw.split('=').collect();
            match a[0] {
                "M" => DestType::M,
                "D" => DestType::D,
                "MD" => DestType::MD,
                "A" => DestType::A,
                "AM" => DestType::AM,
                "AD" => DestType::AD,
                "ADM" => DestType::ADM,
                _ => DestType::Null
            } 
        } else {
            DestType::Null
        }
    }

    pub fn comp(&self) -> &str {
        if self.has_jump() {
            let a: Vec<&str> = self.raw.split(';').collect();
            a[0]
        } else if self.has_eq() {
            let a: Vec<&str> = self.raw.split('=').collect();
            a[1]
        } else {
            ""
        }
    }

    fn has_jump(&self) -> bool {
        if self.command_type() == CommandType::A {
            return false;
        }
        return self.raw.find(';').is_some()
    }

    fn has_eq(&self) -> bool {
        if self.command_type() == CommandType::A {
            return false;
        }
        return self.raw.find('=').is_some()
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

fn parse_line(line: &str) -> Option<Command> {
    let sanitized = line.trim();
    if sanitized.starts_with("//") {
      return None;
    } 
    Some(Command { raw: sanitized.to_string() })
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_line {
        use super::*;

        #[test]
        fn test_include_empty() {
            let query = "  command str ";
            let res = parse_line(&query);
            assert!(res.is_some());
            let command = res.unwrap();
            assert_eq!(
                command.raw,
                "command str"
            );
        }

        #[test]
        fn test_comment() {
            let query = "// some comment";
            let res = parse_line(&query);
            assert!(res.is_none());
        }
    }

    mod command_type {
        use super::*;

        #[test]
        fn type_a() {
            let query = "@100";
            let res = parse_line(&query);
            let command = res.unwrap();
            assert_eq!(
                command.command_type(),
                CommandType::A
            )
        }
    }

    mod symbol {
        use super::*;

        #[test]
        fn success() {
            let query = "@100";
            let res = parse_line(&query);
            let command = res.unwrap();
            assert_eq!(
                command.symbol(),
                "100"
            )
        }
    }

    mod has_jump {
        use super::*;

        #[test]
        fn test_true() {
            let query = "0;JMP";
            let res = parse_line(&query);
            let command = res.unwrap();
            assert!(command.has_jump())
        }

        #[test]
        fn test_false() {
            let comp = "D=M";
            let res_comp = parse_line(&comp);
            assert!(!res_comp.unwrap().has_jump());

            let a_com = "@100";
            let res_a = parse_line(&a_com);
            assert!(!res_a.unwrap().has_jump())
        }
    }

    mod jump {
        use super::*;

        #[test]
        fn success() {
            let q_jgt = "D;JGT";
            let p_jgt = parse_line(&q_jgt);
            assert_eq!(
                p_jgt.unwrap().jump(),
                JumpType::JGT,
            );
            let q_jeq = "D;JEQ";
            let p_jeq = parse_line(&q_jeq);
            assert_eq!(
                p_jeq.unwrap().jump(),
                JumpType::JEQ,
            );
            let q_jge = "D;JGE";
            let p_jge = parse_line(&q_jge);
            assert_eq!(
                p_jge.unwrap().jump(),
                JumpType::JGE,
            );
            let q_jlt = "D;JLT";
            let p_jlt = parse_line(&q_jlt);
            assert_eq!(
                p_jlt.unwrap().jump(),
                JumpType::JLT,
            );
            let q_jne = "D;JNE";
            let p_jne = parse_line(&q_jne);
            assert_eq!(
                p_jne.unwrap().jump(),
                JumpType::JNE,
            );
            let q_jle = "D;JLE";
            let p_jle = parse_line(&q_jle);
            assert_eq!(
                p_jle.unwrap().jump(),
                JumpType::JLE,
            );
            let q_jmp = "0;JMP";
            let p_jmp = parse_line(&q_jmp);
            assert_eq!(
                p_jmp.unwrap().jump(),
                JumpType::JMP,
            );
            let q_null = "M=100";
            let p_null = parse_line(&q_null);
            assert_eq!(
                p_null.unwrap().jump(),
                JumpType::Null,
            );
            let q_null2 = "0;XXX";
            let p_null2 = parse_line(&q_null2);
            assert_eq!(
                p_null2.unwrap().jump(),
                JumpType::Null,
            );
        }
    }

    mod dest {
        use super::*;

        #[test]
        fn jump() {
            let q_jump = "D;JGT";
            let p_jump = parse_line(&q_jump);
            assert_eq!(
                p_jump.unwrap().dest(),
                DestType::Null,
            );
        }

        #[test]
        fn sub_m() {
            let q = "M=M+1";
            let p = parse_line(&q);
            assert_eq!(
                p.unwrap().dest(),
                DestType::M,
            );
        }

        #[test]
        fn sub_d() {
            let q = "D=D+1";
            let p = parse_line(&q);
            assert_eq!(
                p.unwrap().dest(),
                DestType::D,
            );
        }

        #[test]
        fn sub_md() {
            let q = "MD=0";
            let p = parse_line(&q);
            assert_eq!(
                p.unwrap().dest(),
                DestType::MD,
            );
        }

        #[test]
        fn sub_a() {
            let q = "A=0";
            let p = parse_line(&q);
            assert_eq!(
                p.unwrap().dest(),
                DestType::A,
            );
        }


        #[test]
        fn sub_am() {
            let q = "AM=0";
            let p = parse_line(&q);
            assert_eq!(
                p.unwrap().dest(),
                DestType::AM,
            );
        }

        #[test]
        fn sub_ad() {
            let q = "AD=0";
            let p = parse_line(&q);
            assert_eq!(
                p.unwrap().dest(),
                DestType::AD,
            );
        }

        #[test]
        fn sub_adm() {
            let q = "ADM=0";
            let p = parse_line(&q);
            assert_eq!(
                p.unwrap().dest(),
                DestType::ADM,
            );
        }
    }

    mod comp {
        use super::*;

        #[test]
        fn jump() {
            assert_eq!(
                parse_line(&"0;JMP").unwrap().comp(),
                "0",
            );
            assert_eq!(
                parse_line(&"D;JGT").unwrap().comp(),
                "D",
            );
        }

        #[test]
        fn sub() {
            assert_eq!(
                parse_line(&"D=D-A").unwrap().comp(),
                "D-A",
            );
            assert_eq!(
                parse_line(&"M=D+M").unwrap().comp(),
                "D+M",
            );
            assert_eq!(
                parse_line(&"D=A").unwrap().comp(),
                "A",
            );
        }
    }

    mod has_eq {
        use super::*;

        #[test]
        fn test_true() {
            let q = "M=M+1";
            let l = parse_line(&q);
            assert!(l.unwrap().has_eq())
        }

        #[test]
        fn test_false() {
            let q = "0;JMP";
            let l = parse_line(&q);
            assert!(!l.unwrap().has_eq())
        }
    }
}
