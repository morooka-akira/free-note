use std::{
    cell::RefCell,
    collections::{hash_map::DefaultHasher, HashMap},
    hash::Hasher,
    rc::Rc,
};

use downcast_rs::{impl_downcast, Downcast};

use crate::ast::{BlockStatement, Identifier, Node};

pub type ObjectType = String;
pub const INTEGER_OBJ: &str = "INTEGER";
pub const BOOLEAN_OBJ: &str = "BOOLEAN";
pub const NULL_OBJ: &str = "NULL";
pub const RETURN_VALUE_OBJ: &str = "RETURN_VALUE";
pub const ERROR_OBJ: &str = "ERROR";
pub const FUNCTION_OBJ: &str = "FUNCTION";
pub const BUILTIN_OBJ: &str = "BUILTIN";
pub const STRING_OBJ: &str = "STRING";
pub const ARRAY_OBJ: &str = "ARRAY";
pub const HASH_OBJ: &str = "HASH";

pub trait Object: Downcast {
    fn obj_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}
impl_downcast!(Object);

// ------------------------------
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct HashKey {
    object_type: ObjectType,
    value: u64,
}

pub trait Hashable {
    fn hash_key(&self) -> HashKey;
}

// ------------------------------

pub struct Integer {
    pub value: i64,
}

impl Object for Integer {
    fn obj_type(&self) -> ObjectType {
        INTEGER_OBJ.to_string()
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

impl Hashable for Integer {
    fn hash_key(&self) -> HashKey {
        HashKey {
            object_type: self.obj_type(),
            value: self.value as u64,
        }
    }
}

// ------------------------------

pub struct HashPair {
    pub key: Rc<dyn Object>,
    pub value: Rc<dyn Object>,
}

pub struct Hash {
    pub pairs: HashMap<HashKey, HashPair>,
}

impl Object for Hash {
    fn obj_type(&self) -> ObjectType {
        HASH_OBJ.to_string()
    }

    fn inspect(&self) -> String {
        let mut buf = String::new();
        let mut pairs: Vec<String> = vec![];
        for (_, pair) in self.pairs.iter() {
            pairs.push(format!("{}: {}", pair.key.inspect(), pair.value.inspect()));
        }
        pairs.join(", ");
        buf.push('{');
        buf.push_str(pairs.join(", ").as_str());
        buf.push('}');
        buf
    }
}

// ------------------------------

pub struct Boolean {
    pub value: bool,
}

impl Object for Boolean {
    fn obj_type(&self) -> ObjectType {
        BOOLEAN_OBJ.to_string()
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }
}

impl Hashable for Boolean {
    fn hash_key(&self) -> HashKey {
        let mut value: u64 = 0;
        if self.value {
            value = 1;
        }
        HashKey {
            object_type: self.obj_type(),
            value,
        }
    }
}

pub const TRUE: Boolean = Boolean { value: true };
pub const FALSE: Boolean = Boolean { value: false };

// ------------------------------

pub struct Null {}

impl Object for Null {
    fn obj_type(&self) -> ObjectType {
        NULL_OBJ.to_string()
    }

    fn inspect(&self) -> String {
        "null".to_string()
    }
}

pub const NULL: Null = Null {};

// ------------------------------
pub struct ReturnValue {
    pub value: Rc<dyn Object>,
}

impl Object for ReturnValue {
    fn obj_type(&self) -> ObjectType {
        RETURN_VALUE_OBJ.to_string()
    }
    fn inspect(&self) -> String {
        self.value.inspect()
    }
}

// ------------------------------
pub struct Error {
    pub message: String,
}

impl Object for Error {
    fn obj_type(&self) -> ObjectType {
        ERROR_OBJ.to_string()
    }

    fn inspect(&self) -> String {
        "ERROR: ".to_string() + &self.message
    }
}

// ------------------------------
pub struct Function {
    pub parameters: Vec<Rc<Identifier>>,
    pub body: Rc<BlockStatement>,
    pub env: Rc<RefCell<Environment>>,
}

impl Object for Function {
    fn obj_type(&self) -> ObjectType {
        FUNCTION_OBJ.to_string()
    }

    fn inspect(&self) -> String {
        let params = self
            .parameters
            .iter()
            .map(|p| p.string())
            .collect::<Vec<String>>()
            .join(",");

        let mut buf = String::new();
        buf.push_str("fn");
        buf.push('(');
        buf.push_str(&params);
        buf.push_str(") {\n");
        buf.push_str(&self.body.string());
        buf.push('\n');

        buf
    }
}

// ------------------------------
type BuiltinFunction = fn(args: Vec<Rc<dyn Object>>) -> Rc<dyn Object>;
pub struct Builtin {
    pub builtin_function: BuiltinFunction,
}

impl Object for Builtin {
    fn obj_type(&self) -> ObjectType {
        BUILTIN_OBJ.to_string()
    }

    fn inspect(&self) -> String {
        "builtin function".to_string()
    }
}

// ------------------------------
pub struct StringObj {
    pub value: String,
}

impl Object for StringObj {
    fn obj_type(&self) -> ObjectType {
        STRING_OBJ.to_string()
    }

    fn inspect(&self) -> String {
        self.value.clone()
    }
}

impl Hashable for StringObj {
    fn hash_key(&self) -> HashKey {
        let mut hasher: DefaultHasher = DefaultHasher::new();
        hasher.write_str(&self.value);
        HashKey {
            object_type: self.obj_type(),
            value: hasher.finish(),
        }
    }
}

// ------------------------------
pub struct Array {
    pub elements: Vec<Rc<dyn Object>>,
}

impl Object for Array {
    fn obj_type(&self) -> ObjectType {
        ARRAY_OBJ.to_string()
    }

    fn inspect(&self) -> String {
        let mut buf = String::new();
        let elements: Vec<String> = self.elements.iter().map(|e| e.inspect()).collect();
        buf.push('[');
        buf.push_str(&elements.join(", "));
        buf.push(']');
        buf
    }
}

// ------------------------------
#[derive(Default)]
pub struct Environment {
    store: HashMap<String, Rc<dyn Object>>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new_enclosed_environment(outer: Rc<RefCell<Environment>>) -> Self {
        Environment {
            store: HashMap::new(),
            outer: Some(Rc::clone(&outer)),
        }
    }

    pub fn get(&self, name: &str) -> Option<Rc<dyn Object>> {
        let mut obj = self.store.get(name).cloned();
        if obj.is_none() && self.outer.is_some() {
            let outer = self.outer.as_ref().unwrap();
            obj = outer.borrow().get(name);
        }
        obj
    }

    pub fn set(&mut self, name: &str, val: Rc<dyn Object>) -> Option<Rc<dyn Object>> {
        self.store.insert(name.to_string(), val)
    }
}

#[cfg(test)]
mod tests {
    use super::{Hashable, StringObj};

    #[test]
    fn test_string_hash_key() {
        let hello1 = StringObj {
            value: "Hello World".to_string(),
        };
        let hello2 = StringObj {
            value: "Hello World".to_string(),
        };
        let diff1 = StringObj {
            value: "My name is johnny".to_string(),
        };
        let diff2 = StringObj {
            value: "My name is johnny".to_string(),
        };

        assert_eq!(hello1.hash_key(), hello2.hash_key());
        assert_eq!(diff1.hash_key(), diff2.hash_key());
        assert_ne!(hello1.hash_key(), diff1.hash_key());
    }
}
