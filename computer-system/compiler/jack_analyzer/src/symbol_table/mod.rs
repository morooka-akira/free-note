#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum SymbolKind {
    Static,
    Field,
    Arg,
    Var,
    None, // 現在のスコープで見つからない場合
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub index: usize,     // シンボル番号
    pub name: String,     // シンボル名
    pub sym_type: String, // 型
    pub kind: SymbolKind, // 種類
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    scope_index: usize,         // どのスコープのシンボルテーブルを指しているか
    current_table: ScopedTable, // 現在のシンボルテーブル
    tables: Vec<ScopedTable>,   // スコープを持つシンボルテーブル
}

#[derive(Debug, Clone)]
pub struct SymbolIndex {
    static_index: usize,
    field_index: usize,
    arg_index: usize,
    var_index: usize,
}

impl SymbolIndex {
    fn new() -> Self {
        Self {
            static_index: 0,
            field_index: 0,
            arg_index: 0,
            var_index: 0,
        }
    }

    fn current(&self, kind: &SymbolKind) -> usize {
        match kind {
            SymbolKind::Static => self.static_index,
            SymbolKind::Field => self.field_index,
            SymbolKind::Arg => self.arg_index,
            SymbolKind::Var => self.var_index,
            _ => panic!("SymbolKind is None"),
        }
    }

    fn advance(&mut self, kind: &SymbolKind) {
        match kind {
            SymbolKind::Static => self.static_index += 1,
            SymbolKind::Field => self.field_index += 1,
            SymbolKind::Arg => self.arg_index += 1,
            SymbolKind::Var => self.var_index += 1,
            _ => panic!("SymbolKind is None"),
        }
    }
}

#[derive(Debug, Clone)]
struct ScopedTable {
    category: String, // var, argument, field, static, class, subroutine
    symbols: Vec<Symbol>,
    index: SymbolIndex,
}

impl ScopedTable {
    fn new(category: String) -> Self {
        ScopedTable {
            category: category,
            symbols: Vec::new(),
            index: SymbolIndex::new(),
        }
    }

    fn kind_of(&self, name: String) -> SymbolKind {
        let mut kind = SymbolKind::None;
        for symbol in &self.symbols {
            if symbol.name == name {
                kind = symbol.kind.clone();
                break;
            }
        }
        kind
    }

    fn define(&mut self, name: String, sym_type: String, kind: SymbolKind) -> usize {
        let index = self.index.current(&kind);
        self.index.advance(&kind);
        let symbol = Symbol {
            index: index,
            name: name,
            sym_type: sym_type,
            kind: kind,
        };
        self.symbols.push(symbol);
        index
    }

    fn var_count(&self, kind: SymbolKind) -> usize {
        match kind {
            SymbolKind::Static => self.index.static_index,
            SymbolKind::Field => self.index.field_index,
            SymbolKind::Arg => self.index.arg_index,
            SymbolKind::Var => self.index.var_index,
            SymbolKind::None => panic!("SymbolKind is None"),
        }
    }

    fn type_of(&self, name: String) -> Option<String> {
        let mut sym_type: Option<String> = None;
        for symbol in &self.symbols {
            if symbol.name == name {
                sym_type = Some(symbol.sym_type.clone());
            }
        }
        sym_type
    }

    fn index_of(&self, name: String) -> Option<usize> {
        let mut index: Option<usize> = None;
        for symbol in &self.symbols {
            if symbol.name == name {
                index = Some(symbol.index);
            }
        }
        index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_ScopedTable {
        use super::*;

        #[test]
        fn test_kind_of() {
            let mut table = ScopedTable::new("class".to_string());
            table.define("x".to_string(), "int".to_string(), SymbolKind::Field);
            table.define("y".to_string(), "String".to_string(), SymbolKind::Static);
            assert_eq!(table.kind_of("a".to_string()), SymbolKind::None);
            assert_eq!(table.kind_of("x".to_string()), SymbolKind::Field);
            assert_eq!(table.kind_of("y".to_string()), SymbolKind::Static);
        }

        #[test]
        fn test_define() {
            let mut table = ScopedTable::new("class".to_string());
            table.define("x".to_string(), "int".to_string(), SymbolKind::Field);
            assert_eq!(table.symbols.len(), 1);
            assert_eq!(table.index.field_index, 1);
        }

        #[test]
        fn test_var_count() {
            let mut table = ScopedTable::new("class".to_string());
            table.define("x".to_string(), "int".to_string(), SymbolKind::Field);
            table.define("y".to_string(), "int".to_string(), SymbolKind::Field);
            table.define(
                "nAccounts".to_string(),
                "int".to_string(),
                SymbolKind::Static,
            );
            assert_eq!(table.var_count(SymbolKind::Field), 2);
            assert_eq!(table.var_count(SymbolKind::Static), 1);
        }

        #[test]
        fn test_type_of() {
            let mut table = ScopedTable::new("class".to_string());
            table.define("x".to_string(), "int".to_string(), SymbolKind::Field);
            table.define("y".to_string(), "String".to_string(), SymbolKind::Static);
            assert_eq!(table.type_of("x".to_string()).unwrap(), "int");
            assert_eq!(table.type_of("y".to_string()).unwrap(), "String");
            assert_eq!(table.type_of("unknown".to_string()), None);
        }

        #[test]
        fn test_index_of() {
            let mut table = ScopedTable::new("class".to_string());
            table.define("x".to_string(), "int".to_string(), SymbolKind::Field);
            table.define("y".to_string(), "String".to_string(), SymbolKind::Field);
            assert_eq!(table.index_of("x".to_string()).unwrap(), 0);
            assert_eq!(table.index_of("y".to_string()).unwrap(), 1);
            assert_eq!(table.index_of("unknown".to_string()), None);
        }
    }
}
