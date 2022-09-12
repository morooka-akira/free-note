use std::{cell::RefCell, collections::HashMap, rc::Rc};

use downcast_rs::{impl_downcast, Downcast};

use crate::ast::{BlockStatement, Identifier, Node};

pub type ObjectType = String;
pub const INTEGER_OBJ: &str = "INTEGER";
pub const BOOLEAN_OBJ: &str = "BOOLEAN";
pub const NULL_OBJ: &str = "NULL";
pub const RETURN_VALUE_OBJ: &str = "RETURN_VALUE";
pub const ERROR_OBJ: &str = "ERROR_OBJ";
pub const FUNCTION_OBJ: &str = "FUNCTION_OBJ";

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
