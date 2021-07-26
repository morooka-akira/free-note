#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub label_count: i32,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let table = SymbolTable { label_count: 0 };
        table
    }

    // ラベルのインデックスをインクリメントする
    pub fn next_label(&mut self) {
        self.label_count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_label() {
        let mut t = SymbolTable::new();

        assert_eq!(t.label_count, 0);
        t.next_label();
        assert_eq!(t.label_count, 1);
        t.next_label();
        assert_eq!(t.label_count, 2);
    }
}
