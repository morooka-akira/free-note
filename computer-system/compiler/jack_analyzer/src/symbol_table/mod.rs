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
    pub index: String,    // シンボル番号
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
struct ScopedTable {
    category: String, // var, argument, field, static, class, subroutine
    symbols: Vec<Symbol>,
}

impl ScopedTable {
    fn new(category: String) -> Self {
        ScopedTable {
            category: category,
            symbols: Vec::new(),
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
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_ScopedTable {
        use super::*;

        #[test]
        fn test_kind_of() {
            let table = ScopedTable::new("class".to_string());
            assert_eq!(table.kind_of("a".to_string()), SymbolKind::None);
        }
    }
}
