use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use regex::Regex;

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
    AMD,
    Null
}

#[derive(Debug, PartialEq)]
pub enum CompType {
    Zero,
    One,
    NeOne,
    D,
    A,
    NotD,
    NotA,
    NeD,
    NeA,
    Dadd1,
    Aadd1,
    Dsub1,
    Asub1,
    DaddA,
    DsubA,
    AsubD,
    DandA,
    DorA,
    M,
    NotM,
    NeM,
    Madd1,
    Msub1,
    DaddM,
    DsubM,
    MsubD,
    DandM,
    DorM,
    Null
}

#[derive(Debug, Clone)]
pub struct Command {
    raw: String,
}

impl Command {
    pub fn command_type(&self) -> CommandType {
        let re = Regex::new("\\(.+\\)").unwrap();
        if self.raw.starts_with("@") {
            CommandType::A
        } else if re.is_match(&self.raw) {
            CommandType::L
        } else {
            CommandType::C
        }
    }

    pub fn symbol(&self) -> String {
        if self.command_type() == CommandType::A {
            let t: Vec<&str> = self.raw.split('@').collect();
            t[1].to_string()
        } else if self.command_type() == CommandType::L {
            let re = Regex::new("\\((.+)\\)").unwrap();
            let caps = re.captures(&self.raw).unwrap();
            caps[1].to_string()
        } else {
            "".to_string()
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
                "AMD" => DestType::AMD,
                _ => DestType::Null
            } 
        } else {
            DestType::Null
        }
    }

    pub fn comp(&self) -> CompType {
        let c = if self.has_jump() {
            let a: Vec<&str> = self.raw.split(';').collect();
            a[0]
        } else if self.has_eq() {
            let a: Vec<&str> = self.raw.split('=').collect();
            a[1]
        } else {
            ""
        };
        match c {
            "0" => CompType::Zero,
            "1" => CompType::One,
            "-1" => CompType::NeOne,
            "D" => CompType::D,
            "A" => CompType::A,
            "!D" => CompType::NotD,
            "!A" => CompType::NotA,
            "-D" => CompType::NeD,
            "-A" => CompType::NeA,
            "D+1" => CompType::Dadd1,
            "A+1" => CompType::Aadd1,
            "D-1" => CompType::Dsub1,
            "A-1" => CompType::Asub1,
            "D+A" => CompType::DaddA,
            "D-A" => CompType::DsubA,
            "A-D" => CompType::AsubD,
            "D&A" => CompType::DandA,
            "D|A" => CompType::DorA,
            "M" => CompType::M,
            "!M" => CompType::NotM,
            "-M" => CompType::NeM,
            "M+1" => CompType::Madd1,
            "M-1" => CompType::Msub1,
            "D+M" => CompType::DaddM,
            "D-M" => CompType::DsubM,
            "M-D" => CompType::MsubD,
            "D&M" => CompType::DandM,
            "D|M" => CompType::DorM,
            _ => CompType::Null,
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

pub fn parse_line(line: &str) -> Option<Command> {
    if line.starts_with("//") {
      return None;
    } 
    let r = Regex::new(r"//.*").unwrap();
    let t = r.replace(line, "");
    let sanitized = t.trim();
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
            assert_eq!(
                parse_line(&"@100").unwrap().command_type(),
                CommandType::A
            );
        }

        #[test]
        fn type_l() {
            assert_eq!(
                parse_line(&"(LOOP)").unwrap().command_type(),
                CommandType::L
            );
            assert_eq!(
                parse_line(&"(h)").unwrap().command_type(),
                CommandType::L
            )
        }
    }

    mod symbol {
        use super::*;

        #[test]
        fn test_type_a() {
            assert_eq!(
                parse_line(&"@100").unwrap().symbol(),
                "100"
            );
            assert_eq!(
                parse_line(&"@a").unwrap().symbol(),
                "a"
            )
        }

        #[test]
        fn test_type_l() {
            assert_eq!(
                parse_line(&"(LOOP)").unwrap().symbol(),
                "LOOP"
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
            let q = "AMD=0";
            let p = parse_line(&q);
            assert_eq!(
                p.unwrap().dest(),
                DestType::AMD,
            );
        }
    }

    mod comp {
        use super::*;

        #[test]
        fn jump() {
            assert_eq!(
                parse_line(&"0;JMP").unwrap().comp(),
                CompType::Zero,
            );
            assert_eq!(
                parse_line(&"D;JGT").unwrap().comp(),
                CompType::D,
            );
        }

        #[test]
        fn sub() {
            assert_eq!(
                parse_line(&"D=D-A").unwrap().comp(),
                CompType::DsubA,
            );
            assert_eq!(
                parse_line(&"M=D+M").unwrap().comp(),
                CompType::DaddM,
            );
            assert_eq!(
                parse_line(&"D=A").unwrap().comp(),
                CompType::A,
            );
            assert_eq!(
                parse_line(&"DDA").unwrap().comp(),
                CompType::Null,
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
