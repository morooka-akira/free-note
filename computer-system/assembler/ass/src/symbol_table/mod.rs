use std::collections::HashMap;
use crate::parser::Command;
use crate::parser::CommandType;
use std::num::ParseIntError;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    rom: i32,
    ram: i32,
    map: HashMap<String, i32>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut map = SymbolTable {
            rom: 0,
            ram: 16, // RAMのスタートは16から
            map: HashMap::new(),
        };
        map.map.insert("SP".to_string(), 0);
        map.map.insert("LCL".to_string(), 1);
        map.map.insert("ARG".to_string(), 2);
        map.map.insert("THIS".to_string(), 3);
        map.map.insert("THAT".to_string(), 4);
        map.map.insert("R0".to_string(), 0);
        map.map.insert("R1".to_string(), 1);
        map.map.insert("R2".to_string(), 2);
        map.map.insert("R3".to_string(), 3);
        map.map.insert("R4".to_string(), 4);
        map.map.insert("R5".to_string(), 5);
        map.map.insert("R6".to_string(), 6);
        map.map.insert("R7".to_string(), 7);
        map.map.insert("R8".to_string(), 8);
        map.map.insert("R9".to_string(), 9);
        map.map.insert("R10".to_string(), 10);
        map.map.insert("R11".to_string(), 11);
        map.map.insert("R12".to_string(), 12);
        map.map.insert("R13".to_string(), 13);
        map.map.insert("R14".to_string(), 14);
        map.map.insert("R15".to_string(), 15);
        map.map.insert("SCREEN".to_string(), 16384);
        map.map.insert("KBD".to_string(), 24576);
        map
    }

    pub fn map_symbols(&mut self, commands: &Vec<Command>) {
        // NOTE: 1回目のループではラベルをマッピング
        for command in commands {
            if command.command_type() == CommandType::L {
                self.add_label(&command.symbol());
                continue;
            }
            self.next_step();
        }
        // NOTE: 2回目のループでは変数をマッピング
        for command in commands {
            if command.command_type() == CommandType::A {
                let p: Result<i32, ParseIntError> = command.symbol().parse();
                if p.is_err() {
                    self.add_variable(&command.symbol());
                }
            }
        }
    }
    
    // ROMのアドレスをインクリメントする
    pub fn next_step(&mut self) {
        self.rom += 1;
    }

    pub fn add_variable(&mut self, symbol: &str) {
        if self.contains(symbol) {
            return
        }
        self.map.insert(symbol.to_string(), self.ram);
        self.ram += 1;
    }

    pub fn add_label(&mut self, symbol: &str) {
        if self.contains(symbol) {
            return
        }
        self.map.insert(symbol.to_string(), self.rom);
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.map.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &str) -> Option<i32> {
        if !self.contains(symbol) {
            return None 
        }
        Some(*self.map.get(symbol).unwrap())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;

    #[test]
    fn test_map_symbols() {
        let mut t = SymbolTable::new();
        
        let mut commands: Vec<Command> = Vec::new();
        let test_lines = ["@a", "(LOOP)", "@100", "(END)", "@END"];
        for l in test_lines.iter() {
            let c  = parse_line(&l).unwrap();
            commands.push(c.clone());
        }
        t.map_symbols(&commands);
        println!("{:?}", t);
        assert_eq!(t.get_address(&"a").unwrap(), 16);
        assert_eq!(t.get_address(&"LOOP").unwrap(), 1);
        assert_eq!(t.get_address(&"END").unwrap(), 2);
    }

    #[test]
    fn test_next_step() {
        let mut t = SymbolTable::new();
        assert_eq!(t.rom, 0);
        t.next_step();
        assert_eq!(t.rom, 1);
        t.next_step();
        t.next_step();
        assert_eq!(t.rom, 3);
    }

    #[test]
    fn test_add_variable() {
        let mut t = SymbolTable::new();
        t.add_variable("abc");
        assert_eq!(t.map.len(), 24);
        assert_eq!(t.ram, 17);

        t.add_variable("hoge");
        assert_eq!(t.map.len(), 25);
        assert_eq!(t.ram, 18);

        t.add_variable("hoge");
        assert_eq!(t.map.len(), 25);
        assert_eq!(t.ram, 18);
    }

    #[test]
    fn test_add_label() {
        let mut t = SymbolTable::new();
        t.add_label("abc");
        assert_eq!(t.map.len(), 24);
        assert_eq!(*t.map.get("abc").unwrap(), 0);

        t.next_step();
        t.next_step();

        t.add_label("hoge");
        assert_eq!(t.map.len(), 25);
        assert_eq!(*t.map.get("hoge").unwrap(), 2);
    }

    #[test]
    fn test_contains() {
        let mut t = SymbolTable::new();
        t.add_variable("abc");
        assert_eq!(t.contains("abc"), true);
        assert_eq!(t.contains("hoge"), false);
    }

    #[test]
    fn test_get_address() {
        let mut t = SymbolTable::new();
        t.add_variable("abc");
        assert_eq!(t.get_address("abc").unwrap(), 16);

        assert_eq!(t.get_address("hoge"), None);
    }
}
