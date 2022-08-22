use std::{collections::HashMap, rc::Rc};

use downcast_rs::{impl_downcast, Downcast};

pub type ObjectType = String;
pub const INTEGER_OBJ: &str = "INTEGER";
pub const BOOLEAN_OBJ: &str = "BOOLEAN";
pub const NULL_OBJ: &str = "NULL";
pub const RETURN_VALUE_OBJ: &str = "RETURN_VALUE";
pub const ERROR_OBJ: &str = "ERROR_OBJ";

pub trait Object: Downcast {
    fn obj_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}
impl_downcast!(Object);

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
pub struct Environment {
    store: HashMap<String, Box<dyn Object>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&dyn Object> {
        if let Some(obj) = self.store.get(name) {
            Some(obj.as_ref())
        } else {
            None
        }
    }

    pub fn set(&mut self, name: &str, val: Box<dyn Object>) -> Option<Box<dyn Object>> {
        self.store.insert(name.to_string(), val)
    }
}
