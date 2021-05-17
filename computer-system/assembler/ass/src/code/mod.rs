use crate::parser::DestType;
use crate::parser::JumpType;
use crate::parser::CompType;

fn dest(dest_type: DestType) -> String {
    match dest_type {
        DestType::Null => "000",
        DestType::M => "001",
        DestType::D => "010",
        DestType::MD => "011",
        DestType::A => "100",
        DestType::AM => "101",
        DestType::AD => "110",
        DestType::AMD => "111",
    }.to_string()
}

fn jump(jump_type: JumpType) -> String {
    match jump_type {
        JumpType::Null => "000",
        JumpType::JGT => "001",
        JumpType::JEQ => "010",
        JumpType::JGE => "011",
        JumpType::JLT => "100",
        JumpType::JNE => "101",
        JumpType::JLE => "110",
        JumpType::JMP => "111",
    }.to_string()
}

fn comp(comp_type: CompType) -> String {
    match comp_type {
        CompType::Null =>    "0000000",
        CompType::Zero =>    "0101010",
        CompType::One =>     "0111111",
        CompType::NeOne =>   "0111010",
        CompType::D =>       "0001100",
        CompType::A =>       "0110000",
        CompType::NotD =>    "0001101",
        CompType::NotA =>    "0110001",
        CompType::NeD =>     "0001111",
        CompType::NeA =>     "0110011",
        CompType::Dadd1 =>   "0011111",
        CompType::Aadd1 =>   "0110111",
        CompType::Dsub1 =>   "0001110",
        CompType::Asub1 =>   "0110010",
        CompType::DaddA =>   "0000010",
        CompType::DsubA =>   "0010011",
        CompType::AsubD =>   "0000111",
        CompType::DandA =>   "0000000",
        CompType::DorA =>    "0010101",
        CompType::M =>       "1110000",
        CompType::NotM =>    "1110001",
        CompType::NeM =>     "1110011",
        CompType::Madd1 =>   "1110111",
        CompType::Msub1 =>   "1110010",
        CompType::DaddM =>   "1000010",
        CompType::DsubM =>   "1010011",
        CompType::MsubD =>   "1000111",
        CompType::DandM =>   "1000000",
        CompType::DorM =>    "1010101",
    }.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod dest {
        use super::*;

        #[test]
        fn test_dest() {
            assert_eq!(dest(DestType::Null), "000");
            assert_eq!(dest(DestType::M), "001");
            assert_eq!(dest(DestType::D), "010");
            assert_eq!(dest(DestType::MD), "011");
            assert_eq!(dest(DestType::A), "100");
            assert_eq!(dest(DestType::AM), "101");
            assert_eq!(dest(DestType::AD), "110");
            assert_eq!(dest(DestType::AMD), "111");
        }
    }
    mod jump {
        use super::*;

        #[test]
        fn test_jump() {
            assert_eq!(jump(JumpType::Null), "000");
            assert_eq!(jump(JumpType::JGT), "001");
            assert_eq!(jump(JumpType::JEQ), "010");
            assert_eq!(jump(JumpType::JGE), "011");
            assert_eq!(jump(JumpType::JLT), "100");
            assert_eq!(jump(JumpType::JNE), "101");
            assert_eq!(jump(JumpType::JLE), "110");
            assert_eq!(jump(JumpType::JMP), "111");
        }
    }
    mod comp {
        use super::*;

        #[test]
        fn test_comp() {
            assert_eq!(comp(CompType::Null),    "0000000");
            assert_eq!(comp(CompType::Zero),    "0101010");
            assert_eq!(comp(CompType::One),     "0111111");
            assert_eq!(comp(CompType::NeOne),   "0111010");
            assert_eq!(comp(CompType::D),       "0001100");
            assert_eq!(comp(CompType::A),       "0110000");
            assert_eq!(comp(CompType::NotD),    "0001101");
            assert_eq!(comp(CompType::NotA),    "0110001");
            assert_eq!(comp(CompType::NeD),     "0001111");
            assert_eq!(comp(CompType::NeA),     "0110011");
            assert_eq!(comp(CompType::Dadd1),   "0011111");
            assert_eq!(comp(CompType::Aadd1),   "0110111");
            assert_eq!(comp(CompType::Dsub1),   "0001110");
            assert_eq!(comp(CompType::Asub1),   "0110010");
            assert_eq!(comp(CompType::DaddA),   "0000010");
            assert_eq!(comp(CompType::DsubA),   "0010011");
            assert_eq!(comp(CompType::AsubD),   "0000111");
            assert_eq!(comp(CompType::DandA),   "0000000");
            assert_eq!(comp(CompType::DorA),    "0010101");
            assert_eq!(comp(CompType::M),       "1110000");
            assert_eq!(comp(CompType::NotM),    "1110001");
            assert_eq!(comp(CompType::NeM),     "1110011");
            assert_eq!(comp(CompType::Madd1),   "1110111");
            assert_eq!(comp(CompType::Msub1),   "1110010");
            assert_eq!(comp(CompType::DaddM),   "1000010");
            assert_eq!(comp(CompType::DsubM),   "1010011");
            assert_eq!(comp(CompType::MsubD),   "1000111");
            assert_eq!(comp(CompType::DandM),   "1000000");
            assert_eq!(comp(CompType::DorM),    "1010101");
        }
    }
}
