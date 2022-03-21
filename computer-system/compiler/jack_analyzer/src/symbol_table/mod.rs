#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum SymbolKind {
    Static,
    Field,
    Arg,
    Var,
    None, // 現在のスコープで見つからない場合
}

#[derive(Debug)]
pub struct Symbol {
    pub index: usize,     // シンボル番号
    pub name: String,     // シンボル名
    pub sym_type: String, // 型
    pub kind: SymbolKind, // 種類
}

#[derive(Debug)]
pub struct SymbolTable {
    class_table: ScopedTable, // クラススコープを持つシンボルテーブル
    subroutine_table: Option<ScopedTable>, // サブルーチンスコープを持つシンボルテーブル
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            class_table: ScopedTable::new(),
            subroutine_table: None,
        }
    }

    pub fn define(&mut self, name: String, sym_type: String, kind: SymbolKind) {
        self.get_current_table().define(name, sym_type, kind);
    }

    pub fn var_count(&mut self, kind: SymbolKind) -> usize {
        self.get_current_table().var_count(kind)
    }

    pub fn kind_of(&mut self, name: &str) -> SymbolKind {
        self.get_current_table().kind_of(name)
    }

    pub fn type_of(&mut self, name: &str) -> Option<String> {
        self.get_current_table().type_of(name)
    }

    pub fn index_of(&mut self, name: &str) -> Option<usize> {
        self.get_current_table().index_of(name)
    }

    pub fn start_subroutine(&mut self) {
        let table = ScopedTable::new();
        self.subroutine_table = Some(table);
    }

    fn get_current_table(&mut self) -> &mut ScopedTable {
        match &mut self.subroutine_table {
            Some(table) => table,
            None => &mut self.class_table,
        }
    }
}

#[derive(Debug)]
struct ScopedTable {
    symbols: Vec<Symbol>,
    index: SymbolIndex,
}

impl ScopedTable {
    fn new() -> Self {
        ScopedTable {
            symbols: Vec::new(),
            index: SymbolIndex::new(),
        }
    }

    fn kind_of(&self, name: &str) -> SymbolKind {
        let mut kind = SymbolKind::None;
        for symbol in &self.symbols {
            if symbol.name == name {
                kind = symbol.kind.clone();
                break;
            }
        }
        kind
    }

    fn define(&mut self, name: String, sym_type: String, kind: SymbolKind) {
        let index = self.index.current(&kind);
        self.index.advance(&kind);
        let symbol = Symbol {
            index,
            name,
            sym_type,
            kind,
        };
        self.symbols.push(symbol);
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

    fn type_of(&self, name: &str) -> Option<String> {
        let mut sym_type: Option<String> = None;
        for symbol in &self.symbols {
            if symbol.name == name {
                sym_type = Some(symbol.sym_type.clone());
            }
        }
        sym_type
    }

    fn index_of(&self, name: &str) -> Option<usize> {
        let mut index: Option<usize> = None;
        for symbol in &self.symbols {
            if symbol.name == name {
                index = Some(symbol.index);
            }
        }
        index
    }
}

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    mod test_scoped_table {
        use super::*;

        #[test]
        fn test_kind_of() {
            let mut table = ScopedTable::new();
            table.define("x".to_string(), "int".to_string(), SymbolKind::Field);
            table.define("y".to_string(), "String".to_string(), SymbolKind::Static);
            assert_eq!(table.kind_of("a"), SymbolKind::None);
            assert_eq!(table.kind_of("x"), SymbolKind::Field);
            assert_eq!(table.kind_of("y"), SymbolKind::Static);
        }

        #[test]
        fn test_define() {
            let mut table = ScopedTable::new();
            table.define("x".to_string(), "int".to_string(), SymbolKind::Field);
            assert_eq!(table.symbols.len(), 1);
            assert_eq!(table.index.field_index, 1);
        }

        #[test]
        fn test_var_count() {
            let mut table = ScopedTable::new();
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
            let mut table = ScopedTable::new();
            table.define("x".to_string(), "int".to_string(), SymbolKind::Field);
            table.define("y".to_string(), "String".to_string(), SymbolKind::Static);
            assert_eq!(table.type_of("x").unwrap(), "int");
            assert_eq!(table.type_of("y").unwrap(), "String");
            assert_eq!(table.type_of("unknown"), None);
        }

        #[test]
        fn test_index_of() {
            let mut table = ScopedTable::new();
            table.define("x".to_string(), "int".to_string(), SymbolKind::Field);
            table.define("y".to_string(), "String".to_string(), SymbolKind::Field);
            assert_eq!(table.index_of("x").unwrap(), 0);
            assert_eq!(table.index_of("y").unwrap(), 1);
            assert_eq!(table.index_of("unknown"), None);
        }
    }
}
